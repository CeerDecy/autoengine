use crate::types::node::{NodeDefine, NodeRunner, NodeRunnerFactory};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct NodeRegisterBus {
    nodes: HashMap<String, Arc<RwLock<Box<dyn NodeDefine>>>>,
    runner_factories: HashMap<String, Arc<RwLock<Box<dyn NodeRunnerFactory>>>>,
}

impl NodeRegisterBus {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            runner_factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, node: Box<dyn NodeDefine>, factory: Box<dyn NodeRunnerFactory>) {
        let key = node.action_type();

        self.nodes.insert(key.clone(), Arc::new(RwLock::new(node)));
        self.runner_factories
            .insert(key, Arc::new(RwLock::new(factory)));
    }

    pub fn list_nodes(&self) -> Vec<Arc<RwLock<Box<dyn NodeDefine>>>> {
        let mut res = vec![];
        for (_key, value) in self.nodes.iter() {
            res.push(Arc::clone(value));
        }
        res
    }

    pub async fn create_runner(&self, key: &str) -> Option<Box<dyn NodeRunner>> {
        let factory_lock = self.runner_factories.get(key)?.clone();
        let factory = factory_lock.read().await;
        Some(factory.create())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::types::node::{NodeDefine, NodeName, NodeRunner, NodeRunnerFactory};
    use schemars::Schema;
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestNodeDefine {
        action: String,
    }

    impl TestNodeDefine {
        fn new(action: &str) -> Self {
            Self {
                action: action.to_string(),
            }
        }
    }

    impl NodeDefine for TestNodeDefine {
        fn action_type(&self) -> String {
            self.action.clone()
        }

        fn name(&self) -> NodeName {
            NodeName {
                zh: "测试节点".to_string(),
                en: "test node".to_string(),
            }
        }

        fn icon(&self) -> String {
            "icon".to_string()
        }

        fn output_schema(&self) -> Schema {
            true.into()
        }

        fn input_schema(&self) -> Schema {
            true.into()
        }
    }

    struct TestRunnerFactory {
        counter: Arc<AtomicUsize>,
    }

    impl TestRunnerFactory {
        fn new() -> Self {
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn with_counter(counter: Arc<AtomicUsize>) -> Self {
            Self { counter }
        }
    }

    impl NodeRunnerFactory for TestRunnerFactory {
        fn create(&self) -> Box<dyn NodeRunner> {
            Box::new(TestRunner {
                counter: Arc::clone(&self.counter),
            })
        }
    }

    struct TestRunner {
        counter: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl NodeRunner for TestRunner {
        async fn run(&self, _ctx: &Context, _param: serde_json::Value) -> Result<(), String> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn register_and_list_nodes() {
        let mut bus = NodeRegisterBus::new();
        bus.register(
            Box::new(TestNodeDefine::new("action_a")),
            Box::new(TestRunnerFactory::new()),
        );

        let nodes = bus.list_nodes();
        assert_eq!(nodes.len(), 1);

        let node = nodes[0].read().await;
        assert_eq!(node.action_type(), "action_a");
    }

    #[tokio::test]
    async fn create_runner_from_factory() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut bus = NodeRegisterBus::new();
        bus.register(
            Box::new(TestNodeDefine::new("action_b")),
            Box::new(TestRunnerFactory::with_counter(Arc::clone(&counter))),
        );

        let runner = bus
            .create_runner("action_b")
            .await
            .expect("runner should be created");

        let ctx = Context::new(PathBuf::new());
        runner
            .run(&ctx, json!({}))
            .await
            .expect("runner should execute successfully");

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn create_runner_returns_none_for_unknown_key() {
        let bus = NodeRegisterBus::new();
        assert!(bus.create_runner("unknown").await.is_none());
    }
}
