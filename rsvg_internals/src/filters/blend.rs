use cairo;
use markup5ever::{local_name, LocalName};

use crate::drawing_ctx::DrawingCtx;
use crate::error::NodeError;
use crate::node::{NodeResult, NodeTrait, RsvgNode};
use crate::parsers::ParseError;
use crate::property_bag::PropertyBag;
use crate::surface_utils::shared_surface::SharedImageSurface;

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
    Overlay,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    HslHue,
    HslSaturation,
    HslColor,
    HslLuminosity,
}

/// The `feBlend` filter primitive.
pub struct Blend {
    base: PrimitiveWithInput,
    in2: Option<Input>,
    mode: Mode,
}

impl Default for Blend {
    /// Constructs a new `Blend` with empty properties.
    #[inline]
    fn default() -> Blend {
        Blend {
            base: PrimitiveWithInput::new::<Self>(),
            in2: None,
            mode: Mode::Normal,
        }
    }
}

impl NodeTrait for Blend {
    impl_node_as_filter!();

    fn set_atts(&mut self, parent: Option<&RsvgNode>, pbag: &PropertyBag<'_>) -> NodeResult {
        self.base.set_atts(parent, pbag)?;

        for (attr, value) in pbag.iter() {
            match attr {
                local_name!("in2") => {
                    self.in2 = Some(Input::parse(attr, value)?);
                }
                local_name!("mode") => self.mode = Mode::parse(attr, value)?,
                _ => (),
            }
        }

        Ok(())
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
        let input_2 = ctx.get_input(draw_ctx, self.in2.as_ref())?;
        let bounds = self
            .base
            .get_bounds(ctx)
            .add_input(&input)
            .add_input(&input_2)
            .into_irect(draw_ctx);

        // If we're combining two alpha-only surfaces, the result is alpha-only. Otherwise the
        // result is whatever the non-alpha-only type we're working on (which can be either sRGB or
        // linear sRGB depending on color-interpolation-filters).
        let surface_type = if input.surface().is_alpha_only() {
            input_2.surface().surface_type()
        } else {
            if !input_2.surface().is_alpha_only() {
                // All surface types should match (this is enforced by get_input()).
                assert_eq!(
                    input_2.surface().surface_type(),
                    input.surface().surface_type()
                );
            }

            input.surface().surface_type()
        };

        let output_surface = input_2.surface().copy_surface(bounds)?;
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
            cr.set_operator(self.mode.into());
            cr.paint();
        }

        Ok(FilterResult {
            name: self.base.result.clone(),
            output: FilterOutput {
                surface: SharedImageSurface::new(output_surface, surface_type)?,
                bounds,
            },
        })
    }

    #[inline]
    fn is_affected_by_color_interpolation_filters(&self) -> bool {
        true
    }
}

impl Mode {
    fn parse(attr: LocalName, s: &str) -> Result<Self, NodeError> {
        match s {
            "normal" => Ok(Mode::Normal),
            "multiply" => Ok(Mode::Multiply),
            "screen" => Ok(Mode::Screen),
            "darken" => Ok(Mode::Darken),
            "lighten" => Ok(Mode::Lighten),
            "overlay" => Ok(Mode::Overlay),
            "color-dodge" => Ok(Mode::ColorDodge),
            "color-burn" => Ok(Mode::ColorBurn),
            "hard-light" => Ok(Mode::HardLight),
            "soft-light" => Ok(Mode::SoftLight),
            "difference" => Ok(Mode::Difference),
            "exclusion" => Ok(Mode::Exclusion),
            "hue" => Ok(Mode::HslHue),
            "saturation" => Ok(Mode::HslSaturation),
            "color" => Ok(Mode::HslColor),
            "luminosity" => Ok(Mode::HslLuminosity),
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
            Mode::Overlay => cairo::Operator::Overlay,
            Mode::ColorDodge => cairo::Operator::ColorDodge,
            Mode::ColorBurn => cairo::Operator::ColorBurn,
            Mode::HardLight => cairo::Operator::HardLight,
            Mode::SoftLight => cairo::Operator::SoftLight,
            Mode::Difference => cairo::Operator::Difference,
            Mode::Exclusion => cairo::Operator::Exclusion,
            Mode::HslHue => cairo::Operator::HslHue,
            Mode::HslSaturation => cairo::Operator::HslSaturation,
            Mode::HslColor => cairo::Operator::HslColor,
            Mode::HslLuminosity => cairo::Operator::HslLuminosity,			
        }
    }
}
