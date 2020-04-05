// use std::collections::HashMap;

// use sxd_document::Package;
// use sxd_document::dom::Document;
// use sxd_document::parser;
// use sxd_xpath::{Value, Factory};


// pub fn parse<T: AsRef<str>>(xml: T) -> Package {
//     parser::parse(xml.as_ref()).unwrap()
// }

// pub fn evaluate<'d, T: AsRef<str>>(package: &'d Document<'d>, xpath: T) -> Value<'d> {
//     let evaluator = XPathEvaluator::new();
//     evaluator.evaluate(package, xpath.as_ref())
// }

// struct XPathEvaluator<'d> {
//     functions: Functions,
//     variables: Variables<'d>,
//     namespaces: Namespaces,
//     factory: Factory,
// }

// impl<'d> XPathEvaluator<'d> {
//     fn new() -> XPathEvaluator<'d> {
//         let mut fns = HashMap::new();
//         sxd_xpath::function::register_core_functions(&mut fns);
//         XPathEvaluator {
//             functions: fns,
//             variables: HashMap::new(),
//             namespaces: HashMap::new(),
//             factory: Factory::new(),
//         }
//     }

//     fn evaluate(&self, doc: &'d Document<'d>, xpath: &str) -> Value<'d> {
//         let root = doc.root();
//         let context = EvaluationContext::new(
//             root,
//             &self.functions,
//             &self.variables,
//             &self.namespaces,
//         );

//         let xpath = self.factory.build(xpath).unwrap().unwrap();
//         xpath.evaluate(&context).ok().unwrap()
//     }
// }
