use std::cell::{Cell, RefCell};

use cairo;

use attributes::Attribute;
use drawing_ctx::DrawingCtx;
use error::NodeError;
use handle::RsvgHandle;
use node::{NodeResult, NodeTrait, RsvgCNodeImpl, RsvgNode};
use parsers::ParseError;
use property_bag::PropertyBag;
use surface_utils::shared_surface::SharedImageSurface;

use super::context::{FilterContext, FilterOutput, FilterResult};
use super::input::Input;
use super::{Filter, FilterError, PrimitiveWithInput};

/// Enumeration of the possible blending modes.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Mode {
    Normal,
    Multiply,
    Screen,
    Darken,
    Lighten,
}

/// The `feBlend` filter primitive.
pub struct Blend {
    base: PrimitiveWithInput,
    in2: RefCell<Option<Input>>,
    mode: Cell<Mode>,
}

impl Blend {
    /// Constructs a new `Blend` with empty properties.
    #[inline]
    pub fn new() -> Blend {
        Blend {
            base: PrimitiveWithInput::new::<Self>(),
            in2: RefCell::new(None),
            mode: Cell::new(Mode::Normal),
        }
    }
}

impl NodeTrait for Blend {
    fn set_atts(
        &self,
        node: &RsvgNode,
        handle: *const RsvgHandle,
        pbag: &PropertyBag,
    ) -> NodeResult {
        self.base.set_atts(node, handle, pbag)?;

        for (_key, attr, value) in pbag.iter() {
            match attr {
                Attribute::In2 => {
                    self.in2.replace(Some(Input::parse(attr, value)?));
                }
                Attribute::Mode => self.mode.set(Mode::parse(attr, value)?),
                _ => (),
            }
        }

        Ok(())
    }

    #[inline]
    fn get_c_impl(&self) -> *const RsvgCNodeImpl {
        self.base.get_c_impl()
    }
}

impl Filter for Blend {
    fn render(
        &self,
        _node: &RsvgNode,
        ctx: &FilterContext,
        draw_ctx: &mut DrawingCtx,
    ) -> Result<FilterResult, FilterError> {
        let input = self.base.get_input(ctx, draw_ctx)?;
        let input_2 = ctx.get_input(draw_ctx, self.in2.borrow().as_ref())?;
        let bounds = self
            .base
            .get_bounds(ctx)
            .add_input(&input)
            .add_input(&input_2)
            .into_irect(draw_ctx);

        let output_surface = input_2
            .surface()
            .copy_surface(bounds)
            .map_err(FilterError::IntermediateSurfaceCreation)?;
        {
            let cr = cairo::Context::new(&output_surface);
            cr.rectangle(
                bounds.x0 as f64,
                bounds.y0 as f64,
                (bounds.x1 - bounds.x0) as f64,
                (bounds.y1 - bounds.y0) as f64,
            );
            cr.clip();

            input.surface().set_as_source_surface(&cr, 0f64, 0f64);
            cr.set_operator(self.mode.get().into());
            cr.paint();
        }

        Ok(FilterResult {
            name: self.base.result.borrow().clone(),
            output: FilterOutput {
                surface: SharedImageSurface::new(output_surface)
                    .map_err(FilterError::BadIntermediateSurfaceStatus)?,
                bounds,
            },
        })
    }

    #[inline]
    fn is_affected_by_color_interpolation_filters() -> bool {
        true
    }
}

impl Mode {
    fn parse(attr: Attribute, s: &str) -> Result<Self, NodeError> {
        match s {
            "normal" => Ok(Mode::Normal),
            "multiply" => Ok(Mode::Multiply),
            "screen" => Ok(Mode::Screen),
            "darken" => Ok(Mode::Darken),
            "lighten" => Ok(Mode::Lighten),
            _ => Err(NodeError::parse_error(
                attr,
                ParseError::new("invalid value"),
            )),
        }
    }
}

impl From<Mode> for cairo::Operator {
    #[inline]
    fn from(x: Mode) -> Self {
        match x {
            Mode::Normal => cairo::Operator::Over,
            Mode::Multiply => cairo::Operator::Multiply,
            Mode::Screen => cairo::Operator::Screen,
            Mode::Darken => cairo::Operator::Darken,
            Mode::Lighten => cairo::Operator::Lighten,
        }
    }
}
