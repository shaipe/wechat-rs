//! copyright © ecdata.cn 2021 - present
//! Xml操作处理单元
//! created by shaipe 20210228

use sxd_document::dom::Document;
use sxd_document::parser;
use sxd_document::Package;
use sxd_xpath::{Context, Factory, Value, XPath};

pub fn parse<T: AsRef<str>>(xml: T) -> Package {
    parser::parse(xml.as_ref()).unwrap()
}

pub fn evaluate<'d, T: AsRef<str>>(package: &'d Document<'d>, xpath: T) -> Value<'d> {
    let evaluator = XPathEvaluator::new();
    evaluator.evaluate(package, xpath.as_ref())
}

struct XPathEvaluator<'d> {
    context: Context<'d>,
    factory: Factory,
}

impl<'d> XPathEvaluator<'d> {
    fn new() -> XPathEvaluator<'d> {
        let context = Context::new();
        //sxd_xpath::function::register_core_functions(&mut context);
        XPathEvaluator {
            context: context,
            factory: Factory::new(),
        }
    }

    fn evaluate(&self, doc: &'d Document<'d>, path: &str) -> Value<'d> {
        let root = doc.root();
        let xpath: XPath = self
            .factory
            .build(path)
            .expect("Could not compile XPath")
            .expect("No XPath was compiled");
        // let value = evaluate_xpath(doc, path).expect("XPath evaluation failed");
        // value
        xpath
            .evaluate(&self.context, root)
            .ok()
            .expect("XPath evaluation failed")
    }
}
