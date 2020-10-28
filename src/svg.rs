//! SVG directives.

use crate::vdom::{Attributes, CowStr, Nodes, NodesRenderer};
use std::marker::PhantomData;

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

/// Create a virtual SVG element with the given tag name.
#[inline]
pub fn custom<TMsg: 'static>(
    tag_name: impl Into<CowStr>,
    attr: impl Attributes<TMsg>,
    children: impl Nodes<TMsg>,
) -> impl Nodes<TMsg> {
    SvgElement {
        tag_name: tag_name.into(),
        attr,
        children,
        _marker: PhantomData,
    }
}

struct SvgElement<TMsg, A, C> {
    tag_name: CowStr,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Nodes<TMsg> for SvgElement<TMsg, A, C>
where
    A: Attributes<TMsg>,
    C: Nodes<TMsg>,
{
    #[inline]
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.element(
            self.tag_name,
            Some(SVG_NAMESPACE_URI.into()),
            self.attr,
            self.children,
        )?;
        renderer.end()
    }
}

macro_rules! svg_elements {
    ($( $name:ident => $tag_name:expr ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a virtual SVG element corresponding to [`<"
                    $tag_name
                    ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/"
                    $tag_name
                    ")."]
            #[inline]
            pub fn $name<TMsg: 'static>(
                attr: impl Attributes<TMsg>,
                children: impl Nodes<TMsg>,
            ) -> impl Nodes<TMsg> {
                custom($tag_name, attr, children)
            }
        }
    )*};
}

// ref: https://developer.mozilla.org/en-US/docs/Web/SVG/Element#SVG_elements_A_to_Z
svg_elements! {
    a => "a",
    animate => "animate",
    animate_motion => "animateMotion",
    animate_transform => "animateTransform",
    circle => "circle",
    clip_path => "clipPath",
    // ignored: color-profile
    defs => "defs",
    desc => "desc",
    discard => "discard",
    ellipse => "ellipse",
    fe_blend => "feBlend",
    fe_color_matrix => "feColorMatrix",
    fe_component_transfer => "feComponentTransfer",
    fe_composite => "feComposite",
    fe_convolve_matrix => "feConvolveMatrix",
    fe_diffuse_lighting => "feDiffuseLighting",
    fe_displacement_map => "feDisplacementMap",
    fe_drop_shadow => "feDropShadow",
    fe_flood => "feFlood",
    fe_func_a => "feFuncA",
    fe_func_b => "feFuncB",
    fe_func_g => "feFuncG",
    fe_func_r => "feFuncR",
    fe_gaussian_blur => "feGaussianBlur",
    fe_image => "feImage",
    fe_merge => "feMerge",
    fe_merge_node => "feMergeNode",
    fe_morphology => "feMorphology",
    fe_offset => "feOffset",
    fe_point_light => "fePointLight",
    fe_specular_lighting => "feSpecularLighting",
    fe_spot_light => "feSpotLight",
    fe_tile => "feTile",
    fe_turbulence => "feTurbulence",
    filter => "filter",
    foreign_object => "foreignObject",
    g => "g",
    hatch => "hatch",
    hatchpath => "hatchpath",
    image => "image",
    line => "line",
    linear_gradient => "linearGradient",
    marker => "marker",
    mask => "mask",
    // ignored: mesh, meshgradient, meshpatch, meshrow
    metadata => "metadata",
    mpath => "mpath",
    path => "path",
    pattern => "pattern",
    polygon => "polygon",
    polyline => "polyline",
    radial_gradient => "radialGradient",
    rect => "rect",
    // ignored: script
    set => "set",
    // ignored: solidcolor
    stop => "stop",
    style => "style",
    svg => "svg",
    switch => "switch",
    symbol => "symbol",
    text => "text",
    text_path => "textPath",
    title => "title",
    tspan => "tspan",
    // ignored: unknown
    use_ => "use",
    view => "view",
}

/// SVG attributes.
pub mod attr {
    use crate::vdom::{attribute, Attributes, CowStr};

    macro_rules! svg_attributes {
        ( $( $name:ident => $attrname:expr, )* ) => {$(
            paste::paste! {
                #[doc = "Create an `Attributes` that specifies [`"
                        $attrname
                        "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"
                        $attrname
                        ") attribute."]
                pub fn $name<TMsg: 'static>(val: impl Into<CowStr>) -> impl Attributes<TMsg> {
                    attribute($attrname, val.into())
                }
            }
        )*};
    }

    // ref: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#SVG_attributes_A_to_Z
    svg_attributes! {
        // ignored: accent-height
        accumulate => "accumulate",
        additive => "additive",
        alignment_baseline => "alignment-baseline",
        // ignored: allowReorder, alphabetic
        amplitude => "amplitude",
        // ignored: arabic-form, ascent
        attribute_name => "attributeName",
        attribute_type => "attributeType",
        // ignored: autoReverse
        azimuth => "azimuth",
        base_frequency => "baseFrequency",
        baseline_shift => "baseline-shift",
        // ignored: baseProfile, bbox
        begin => "begin",
        bias => "bias",
        by => "by",
        calc_mode => "calcMode",
        // ignored: cap-height
        // omitted: class (provided by vdom)
        // ignored: clip
        clip_path_units => "clipPathUnits",
        clip_path => "clip-path",
        clip_rule => "clip-rule",
        color => "color",
        color_interpolation => "color-interpolation",
        color_interpolation_filters => "color-interpolation-filters",
        // ignored: color-profile, color-rendering, contentScriptType, contentStyleType
        cursor => "cursor",
        cx => "cx",
        cy => "cy",
        d => "d",
        // ignored: decelerate, descent
        diffuse_constant => "diffuseConstant",
        direction => "direction",
        display => "display",
        divisor => "divisor",
        dominant_baseline => "dominant-baseline",
        dur => "dur",
        dx => "dx",
        dy => "dy",
        edge_mode => "edgeMode",
        elevation => "elevation",
        // ignored: enable-background
        end => "end",
        exponent => "exponent",
        // ignored: externalResourcesRequired
        fill => "fill",
        fill_opacity => "fixx-opacity",
        fill_rule => "fill-rule",
        filter => "filter",
        // ignored: filterRes
        filter_units => "filterUnits",
        flood_color => "flood-color",
        flood_opacity => "flood-opacity",
        font_family => "font-family",
        font_size => "font-size",
        font_size_adjust => "font-size-adjust",
        font_stretch => "font-stretch",
        font_style => "font-style",
        font_variant => "font-variant",
        font_weight => "font-weight",
        // ignored: format
        from => "from",
        fr => "fr",
        fx => "fx",
        fy => "fy",
        // ignored: g1, g2, glyph-name, glyph-orientation-horizontal, glyph-orientation-vertical, glyphRef
        gradient_transform => "gradientTransform",
        gradient_units => "gradientUnits",
        // ignored: hanging
        height => "height",
        href => "href",
        // ignored: hreflang, horiz-adv-x, horiz-origin-x
        id => "id",
        // ignored: ideographic
        image_rendering => "image-rendering",
        in_ => "in",
        in2 => "in2",
        // ignored: k
        k1 => "k1",
        k2 => "k2",
        k3 => "k3",
        k4 => "k4",
        kernel_matrix => "kernelMatrix",
        // ignored: kernelUnitLength, kerning
        key_points => "keyPoints",
        key_splines => "keySplines",
        key_times => "keyTimes",
        lang => "lang",
        length_adjust => "lengthAdjust",
        letter_spacing => "letter-spacing",
        lighting_color => "lighting-color",
        limiting_cone_angle => "limitingConeAngle",
        // ignored: local
        marked_end => "marker-end",
        marker_mid => "marker-mid",
        marker_start => "marker-start",
        marker_height => "markerHeight",
        marker_units => "markerUnits",
        marker_width => "markerWidth",
        mask => "mask",
        mask_content_units => "maskContentUnits",
        mask_units => "maskUnits",
        // ignored: mathematical
        max => "max",
        media => "media",
        method => "method",
        min => "min",
        mode => "mode",
        // ignored: name
        num_octaves => "numOctaves",
        // ignored: offset
        opacity => "opacity",
        operator => "operator",
        order => "order",
        orient => "orient",
        // ignored: orientation
        origin => "origin",
        overflow => "overflow",
        overline_position => "overline-position",
        overline_thickness => "overline-thickness",
        // ignored: panose-1
        paint_order => "paint-order",
        path => "path",
        path_length => "pathLength",
        pattern_content_units => "patternContentUnits",
        pattern_transform => "patternTransform",
        pattern_units => "patternUnits",
        // ignored: ping
        points => "points",
        points_at_x => "pointsAtX",
        points_at_y => "pointsAtY",
        points_at_z => "pointsAtZ",
        preserve_alpha => "preserveAlpha",
        preserve_aspect_ratio => "preserve_aspect_ratio",
        primitive_units => "primitiveUnits",
        r => "r",
        radius => "radius",
        // ignored: referrerPolicy
        ref_x => "refX",
        ref_y => "refY",
        // ignored: rel, rendering-intent
        repeat_count => "repeatCount",
        repeat_dur => "repeatDur",
        // ignored: requiredExtensions, requiredFeatures
        restart => "restart",
        result => "result",
        rotate => "rotate",
        rx => "rx",
        ry => "ry",
        scale => "scale",
        seed => "seed",
        shape_rendering => "shape-rendering",
        // ignored: slope
        spacing => "spacing",
        specular_constant => "specularConstant",
        specular_exponent => "specularExponent",
        // ignored: speed
        spread_method => "spreadMethod",
        start_offset => "startOffset",
        std_deviation => "stdDeviation",
        // ignored: stemh, stemv
        stitch_tiles => "stitchTiles",
        stop_color => "stop-color",
        stop_opacity => "stop-opacity",
        strikethrough_position => "strikethrough-position",
        strikethrough_thickness => "strikethrough-thickness",
        // ignroed: string
        stroke => "stroke",
        stroke_dasharray => "stroke-dasharray",
        stroke_dashoffset => "stroke-dashoffset",
        stroke_linecap => "stroke-linecap",
        stroke_linejoin => "stroke-linejoin",
        stroke_miterlimit => "stroke-miterlimit",
        stroke_opacity => "stroke-opacity",
        stroke_width => "stroke-width",
        // ignored: style (provided by vdom)
        surface_scale => "surfaceScale",
        system_language => "systemLanguage",
        tabindex => "tabindex",
        table_values => "tableValues",
        target => "target",
        target_x => "targetX",
        target_y => "targetY",
        text_anchor => "text-anchor",
        text_decoration => "text-decoration",
        text_rendering => "text-rendering",
        text_length => "textLength",
        to => "to",
        transform => "transform",
        // ignored: transform-origin
        type_ => "type",
        // ignored: u1, u2
        underline_position => "underline-position",
        underline_thickness => "underline-thickness",
        // ignored: unicode
        unicode_bidi => "unicode-bidi",
        // ignored: unicode-range, units-per-em
        // ignored: v-alphabetic, v-hanging, v-ideographic, v-mathematical
        values => "values",
        vector_effect => "vector-effect",
        // ignored: version, vert-adv-y, vert-origin-x, vert-origin-y
        view_box => "viewBox",
        // ignored: viewTarget
        visibility => "visibility",
        width => "width",
        // ignored: widths
        word_spacing => "word-spacing",
        writing_mode => "writing-mode",
        x => "x",
        // ignroed: x-height
        x1 => "x1",
        x2 => "x2",
        x_channel_selector => "xChannelSelector",
        // ignored: xlink:actuate, xlink:arcrole, xlink:href, xlink:role, xlink:show, xlink:title, xlink:type
        // ignored: xml:base, xml:lang, xml:space
        y => "y",
        y1 => "y1",
        y2 => "y2",
        y_channel_selector => "yChannelSelector",
        z => "z",
        // ignored: zoomAndPan
    }
}
