use std::collections::HashMap;

use sxd_document::Package;
use sxd_document::dom::Document;
use sxd_document::parser;
use sxd_xpath::{Value, Factory,Context};


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

    fn evaluate(&self, doc: &'d Document<'d>, xpath: &str) -> Value<'d> {
        let root = doc.root();
        let xpath = self.factory.build(xpath).unwrap().unwrap();
        xpath.evaluate(&self.context,root).ok().unwrap()
    }
}
