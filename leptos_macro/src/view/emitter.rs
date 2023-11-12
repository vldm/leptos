use quote::ToTokens;
use rstml::node::KeyedAttribute;
trait AttributeEmitter {
    /// Handle class attribute, one by one,
    /// emitter implementation decide whenever to collect them and process in `push_end_attributes`
    /// or iteratively emit.
    // `class = ""  class:foo=true, class=(x, "bar")`
    fn push_class_attribute(&mut self, attribute: &KeyedAttribute);
    /// Handle style attribute, same as in class, one by one.
    // `style = ""  style:foo=true, style=(x, "bar")`
    fn push_style_attribute(&mut self, attribute: &KeyedAttribute);

    fn push_ref_attribute(&mut self, attribute: &KeyedAttribute);
    // if name == "ref" || name == "_ref" || name == "ref_" || name == "node_ref" {
    // prop: 
    // attr:

    // `on:click=|_| foo()`
    fn push_event_attribute(&mut self, attribute: &KeyedAttribute);
    // `id = "1" x = || y inner_html = "<div></div>"`
    fn push_other_attribute(&mut self, attribute: &KeyedAttribute);
    // >
    fn push_end_attributes(&mut self);
}


// pub fn emit_attribute()

    // // `<br/> <div> .. </div> ..`
    // fn push_child(&mut self, node: &Node);
    // // ``(inner_html)? (</div>)?`
    // fn push_close_tag(&mut self, close_tag_name: Option<&NodeName>);
    // fn into_result(self, element: &NodeElement) -> Self::Result;