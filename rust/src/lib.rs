#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate downcast_rs;

pub use aspect_ratio::{
    FitMode,
    AlignMode,
    Align,
    AspectRatio,
    ParseAspectRatioError,
    rsvg_aspect_ratio_parse,
    rsvg_aspect_ratio_compute
};

pub use bbox::{
    RsvgBbox,
    rsvg_bbox_init,
    rsvg_bbox_insert,
    rsvg_bbox_clip
};

pub use cnode::{
    rsvg_rust_cnode_new,
    rsvg_rust_cnode_get_impl
};

pub use gradient::{
    gradient_linear_new,
    gradient_radial_new,
    gradient_destroy,
    gradient_add_color_stop,
    gradient_resolve_fallbacks_and_set_pattern
};

pub use length::{
    LengthUnit,
    LengthDir,
    RsvgLength,
    rsvg_length_parse,
    rsvg_length_normalize,
    rsvg_length_hand_normalize,
};

pub use marker::{
    rsvg_node_marker_new,
};

pub use node::{
    rsvg_node_get_type,
    rsvg_node_get_parent,
    rsvg_node_ref,
    rsvg_node_unref,
    rsvg_node_is_same,
    rsvg_node_get_state,
    rsvg_node_add_child,
    rsvg_node_set_atts,
    rsvg_node_draw,
    rsvg_node_foreach_child,
};

pub use path_builder::{
    rsvg_path_builder_new,
    rsvg_path_builder_destroy,
    rsvg_path_builder_move_to,
    rsvg_path_builder_line_to,
    rsvg_path_builder_curve_to,
    rsvg_path_builder_close_path,
    rsvg_path_builder_arc,
    rsvg_path_builder_add_to_cairo_context
};

pub use path_parser::{
    rsvg_path_parser_from_str_into_builder
};

pub use pattern::{
    pattern_new,
    pattern_destroy,
    pattern_resolve_fallbacks_and_set_pattern,
};

pub use shapes::{
    rsvg_node_circle_new,
    rsvg_node_ellipse_new,
    rsvg_node_line_new,
    rsvg_node_path_new,
    rsvg_node_polygon_new,
    rsvg_node_polyline_new,
    rsvg_node_rect_new,
};
pub use viewbox::{
    RsvgViewBox
};


mod aspect_ratio;
mod bbox;
mod cnode;
mod drawing_ctx;
mod handle;
mod gradient;
mod length;
mod marker;
mod node;
mod parsers;
mod path_builder;
mod path_parser;
mod pattern;
mod property_bag;
mod state;
mod shapes;
mod util;
mod viewbox;