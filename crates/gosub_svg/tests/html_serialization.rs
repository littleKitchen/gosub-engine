//! The HTML writer's SVG consumer must receive XML, including for mixed HTML/SVG trees.

use gosub_css3::system::Css3System;
use gosub_html5::document::document_impl::DocumentImpl;
use gosub_html5::html_compile;
use gosub_html5::parser::Html5Parser;
use gosub_interface::config::ModuleConfiguration;
use gosub_interface::document::Document;
use gosub_svg::SVGDocument;

#[derive(Clone, Debug, PartialEq)]
struct Config;

impl ModuleConfiguration for Config {
    type CssSystem = Css3System;
    type Document = DocumentImpl<Self>;
    type HtmlParser = Html5Parser<'static, Self>;
}

#[test]
fn an_inline_svg_with_escaped_values_and_foreign_html_decodes() -> Result<(), Box<dyn std::error::Error>> {
    let doc = html_compile::<Config>(
        r#"<svg id="target" xmlns="http://www.w3.org/2000/svg" width="20" height="10">
          <desc data-value="&amp;&quot;&lt;&gt;&nbsp;">A &amp; B &lt; C &gt; D &nbsp; &amp;nbsp;</desc>
          <foreignObject><div><br><script>if (a < b && b > c) "&nbsp;";</script></div></foreignObject>
          <rect width="20" height="10"/>
        </svg>"#,
    );
    let id = doc.node_by_named_id("target").ok_or("inline SVG was not parsed")?;
    let svg = SVGDocument::from_html_doc::<Config>(id, doc)?;
    assert_eq!(svg.tree.size().width(), 20.0);
    assert_eq!(svg.tree.size().height(), 10.0);
    assert!(svg.tree.root().has_children(), "the rectangle must survive decoding");
    Ok(())
}
