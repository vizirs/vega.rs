// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vega {
    pub axes: Option<Vec<Axis>>,
    pub data: Option<Vec<Data>>,
    pub encode: Option<VegaEncode>,
    pub layout: Option<Layout>,
    pub legends: Option<Vec<Legend>>,
    pub marks: Option<Vec<Mark>>,
    pub projections: Option<Vec<Projection>>,
    pub scales: Option<Vec<Scale>>,
    pub signals: Option<Vec<Signal>>,
    pub title: Option<Title>,
    pub usermeta: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub autosize: Option<Autosize>,
    pub background: Option<BackgroundElement>,
    pub config: Option<HashMap<String, Option<serde_json::Value>>>,
    pub description: Option<String>,
    pub height: Option<HeightElement>,
    pub padding: Option<Padding>,
    pub style: Option<Style>,
    pub width: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutosizeSignalRef {
    pub contains: Option<Contains>,
    pub resize: Option<bool>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<AutosizeEnum>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Axis {
    pub aria: Option<bool>,
    #[serde(rename = "bandPosition")]
    pub band_position: Box<Option<Box<AngleUnion>>>,
    pub description: Option<String>,
    pub domain: Option<bool>,
    #[serde(rename = "domainCap")]
    pub domain_cap: Option<FontUnion>,
    #[serde(rename = "domainColor")]
    pub domain_color: Option<ColorValue>,
    #[serde(rename = "domainDash")]
    pub domain_dash: Option<DomainDashUnion>,
    #[serde(rename = "domainDashOffset")]
    pub domain_dash_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "domainOpacity")]
    pub domain_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "domainWidth")]
    pub domain_width: Box<Option<Box<AngleUnion>>>,
    pub encode: Option<AxeEncode>,
    pub format: Option<AxeFormat>,
    #[serde(rename = "formatType")]
    pub format_type: Option<FormatTypeUnion>,
    pub grid: Option<bool>,
    #[serde(rename = "gridCap")]
    pub grid_cap: Option<FontUnion>,
    #[serde(rename = "gridColor")]
    pub grid_color: Option<ColorValue>,
    #[serde(rename = "gridDash")]
    pub grid_dash: Option<DomainDashUnion>,
    #[serde(rename = "gridDashOffset")]
    pub grid_dash_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "gridOpacity")]
    pub grid_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "gridScale")]
    pub grid_scale: Option<String>,
    #[serde(rename = "gridWidth")]
    pub grid_width: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelAlign")]
    pub label_align: Option<AlignUnion>,
    #[serde(rename = "labelAngle")]
    pub label_angle: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelBaseline")]
    pub label_baseline: Option<BaselineUnion>,
    #[serde(rename = "labelBound")]
    pub label_bound: Option<LabelBound>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<ColorValue>,
    #[serde(rename = "labelFlush")]
    pub label_flush: Option<LabelBound>,
    #[serde(rename = "labelFlushOffset")]
    pub label_flush_offset: Option<HeightElement>,
    #[serde(rename = "labelFont")]
    pub label_font: Option<FontUnion>,
    #[serde(rename = "labelFontSize")]
    pub label_font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelFontStyle")]
    pub label_font_style: Option<FontUnion>,
    #[serde(rename = "labelFontWeight")]
    pub label_font_weight: Option<FontWeightUnion>,
    #[serde(rename = "labelLimit")]
    pub label_limit: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelLineHeight")]
    pub label_line_height: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOffset")]
    pub label_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOpacity")]
    pub label_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOverlap")]
    pub label_overlap: Option<LabelOverlap>,
    #[serde(rename = "labelPadding")]
    pub label_padding: Box<Option<Box<AngleUnion>>>,
    pub labels: Option<bool>,
    #[serde(rename = "labelSeparation")]
    pub label_separation: Option<HeightElement>,
    #[serde(rename = "maxExtent")]
    pub max_extent: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "minExtent")]
    pub min_extent: Box<Option<Box<AngleUnion>>>,
    pub offset: Box<Option<Box<AngleUnion>>>,
    pub orient: AxeOrient,
    pub position: Box<Option<Box<AngleUnion>>>,
    pub scale: String,
    #[serde(rename = "tickBand")]
    pub tick_band: Option<TickBand>,
    #[serde(rename = "tickCap")]
    pub tick_cap: Option<FontUnion>,
    #[serde(rename = "tickColor")]
    pub tick_color: Option<ColorValue>,
    #[serde(rename = "tickCount")]
    pub tick_count: Option<TickCount>,
    #[serde(rename = "tickDash")]
    pub tick_dash: Option<DomainDashUnion>,
    #[serde(rename = "tickDashOffset")]
    pub tick_dash_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "tickExtra")]
    pub tick_extra: Option<TickExtraUnion>,
    #[serde(rename = "tickMinStep")]
    pub tick_min_step: Option<HeightElement>,
    #[serde(rename = "tickOffset")]
    pub tick_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "tickOpacity")]
    pub tick_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "tickRound")]
    pub tick_round: Option<TickRoundUnion>,
    pub ticks: Option<bool>,
    #[serde(rename = "tickSize")]
    pub tick_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "tickWidth")]
    pub tick_width: Box<Option<Box<AngleUnion>>>,
    pub title: Option<TextOrSignal>,
    #[serde(rename = "titleAlign")]
    pub title_align: Option<AlignUnion>,
    #[serde(rename = "titleAnchor")]
    pub title_anchor: Option<AnchorUnion>,
    #[serde(rename = "titleAngle")]
    pub title_angle: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleBaseline")]
    pub title_baseline: Option<BaselineUnion>,
    #[serde(rename = "titleColor")]
    pub title_color: Option<ColorValue>,
    #[serde(rename = "titleFont")]
    pub title_font: Option<FontUnion>,
    #[serde(rename = "titleFontSize")]
    pub title_font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleFontStyle")]
    pub title_font_style: Option<FontUnion>,
    #[serde(rename = "titleFontWeight")]
    pub title_font_weight: Option<FontWeightUnion>,
    #[serde(rename = "titleLimit")]
    pub title_limit: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleLineHeight")]
    pub title_line_height: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleOpacity")]
    pub title_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titlePadding")]
    pub title_padding: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleX")]
    pub title_x: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleY")]
    pub title_y: Box<Option<Box<AngleUnion>>>,
    pub translate: Box<Option<Box<AngleUnion>>>,
    pub values: Option<ArrayOrSignal>,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AngleClass {
    pub band: Option<Band>,
    pub exponent: Box<Option<Box<AngleUnion>>>,
    pub extra: Option<bool>,
    pub mult: Box<Option<Box<AngleUnion>>>,
    pub offset: Box<Option<Box<AngleUnion>>>,
    pub round: Option<bool>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AngleElement {
    pub test: Option<String>,
    pub band: Option<Band>,
    pub exponent: Box<Option<Box<AngleUnion>>>,
    pub extra: Option<bool>,
    pub mult: Box<Option<Box<AngleUnion>>>,
    pub offset: Box<Option<Box<AngleUnion>>>,
    pub round: Option<bool>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleSignalRef {
    pub signal: Option<String>,
    pub datum: Box<Option<Box<Field>>>,
    pub group: Box<Option<Box<Field>>>,
    pub level: Option<f64>,
    pub parent: Box<Option<Box<Field>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseColorValue {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
    pub count: Option<f64>,
    pub gradient: Box<Option<Box<Field>>>,
    pub start: Option<Vec<f64>>,
    pub stop: Option<Vec<f64>>,
    pub color: Option<Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub b: Option<NumberValue>,
    pub g: Option<NumberValue>,
    pub r: Option<NumberValue>,
    pub h: Option<NumberValue>,
    pub l: Option<NumberValue>,
    pub s: Option<NumberValue>,
    pub a: Option<NumberValue>,
    pub c: Option<NumberValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleBaseColorValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
    pub count: Option<f64>,
    pub gradient: Box<Option<Box<Field>>>,
    pub start: Option<Vec<f64>>,
    pub stop: Option<Vec<f64>>,
    pub color: Option<Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainDashClass {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AxeEncode {
    pub axis: Option<GuideEncode>,
    pub domain: Option<GuideEncode>,
    pub grid: Option<GuideEncode>,
    pub labels: Option<GuideEncode>,
    pub ticks: Option<GuideEncode>,
    pub title: Option<GuideEncode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuideEncode {
    pub interactive: Option<bool>,
    pub name: Option<String>,
    pub style: Option<Style>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffySignalRef {
    pub date: Option<String>,
    pub day: Option<String>,
    pub hours: Option<String>,
    pub milliseconds: Option<String>,
    pub minutes: Option<String>,
    pub month: Option<String>,
    pub quarter: Option<String>,
    pub seconds: Option<String>,
    pub week: Option<String>,
    pub year: Option<String>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatTypeSignalRef {
    pub signal: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlignElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlignValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaselineElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaselineValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontWeightElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontWeightValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickCountSignalRef {
    pub interval: Option<Interval>,
    pub step: Option<HeightElement>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickRoundElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub on: Option<Vec<OnTrigger>>,
    pub transform: Option<Vec<Transform>>,
    pub source: Option<Style>,
    pub async: Option<TickExtraUnion>,
    pub format: Option<SignalRef>,
    pub url: Option<BackgroundElement>,
    pub values: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef {
    pub parse: Option<ParseUnion>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<BackgroundElement>,
    pub copy: Option<TickExtraUnion>,
    pub property: Option<BackgroundElement>,
    pub header: Option<Vec<String>>,
    pub delimiter: Option<String>,
    pub feature: Option<BackgroundElement>,
    pub filter: Option<Filter>,
    pub mesh: Option<BackgroundElement>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseSignalRef {
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnTrigger {
    pub insert: Option<String>,
    pub modify: Option<String>,
    pub remove: Option<Remove>,
    pub toggle: Option<String>,
    pub trigger: String,
    pub values: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    pub fields: Option<FieldsUnion>,
    pub query: Option<ArrayOrSignal>,
    pub signal: Option<String>,
    #[serde(rename = "type")]
    pub transform_type: TransformType,
    pub filter: Option<serde_json::Value>,
    pub ignore: Option<HeightElement>,
    #[serde(rename = "as")]
    pub transform_as: Option<AsUnion>,
    pub orient: Option<TransformOrient>,
    pub require: Option<FormatTypeSignalRef>,
    pub shape: Option<ShapeUnion>,
    #[serde(rename = "sourceX")]
    pub source_x: Option<ColorUnion>,
    #[serde(rename = "sourceY")]
    pub source_y: Option<ColorUnion>,
    #[serde(rename = "targetX")]
    pub target_x: Option<ColorUnion>,
    #[serde(rename = "targetY")]
    pub target_y: Option<ColorUnion>,
    #[serde(rename = "endAngle")]
    pub end_angle: Option<HeightElement>,
    pub field: Option<ColorUnion>,
    pub sort: Option<SortUnion>,
    #[serde(rename = "startAngle")]
    pub start_angle: Option<HeightElement>,
    pub groupby: Option<GroupbyUnion>,
    pub offset: Option<TransformOffset>,
    pub alpha: Option<HeightElement>,
    #[serde(rename = "alphaMin")]
    pub alpha_min: Option<HeightElement>,
    #[serde(rename = "alphaTarget")]
    pub alpha_target: Option<HeightElement>,
    pub forces: Option<Vec<ForceElement>>,
    pub iterations: Option<HeightElement>,
    pub restart: Option<TickExtraUnion>,
    #[serde(rename = "static")]
    pub transform_static: Option<TickExtraUnion>,
    #[serde(rename = "velocityDecay")]
    pub velocity_decay: Option<HeightElement>,
    pub bandwidth: Option<StepsUnion>,
    #[serde(rename = "cellSize")]
    pub cell_size: Option<HeightElement>,
    pub count: Option<HeightElement>,
    pub nice: Option<TickExtraUnion>,
    pub size: Option<StepsUnion>,
    pub smooth: Option<TickExtraUnion>,
    pub thresholds: Option<CenterUnion>,
    pub values: Option<ValuesUnion>,
    pub weight: Option<ColorUnion>,
    pub x: Option<ColorUnion>,
    pub y: Option<ColorUnion>,
    pub geojson: Option<ColorUnion>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<FontSizeUnion>,
    pub projection: Option<String>,
    pub extent: Option<ArrayOrSignal>,
    #[serde(rename = "extentMajor")]
    pub extent_major: Option<ArrayOrSignal>,
    #[serde(rename = "extentMinor")]
    pub extent_minor: Option<ArrayOrSignal>,
    pub precision: Option<HeightElement>,
    pub step: Option<StepsUnion>,
    #[serde(rename = "stepMajor")]
    pub step_major: Option<CenterUnion>,
    #[serde(rename = "stepMinor")]
    pub step_minor: Option<CenterUnion>,
    pub color: Option<ColorUnion>,
    pub opacity: Option<FontSizeUnion>,
    pub resolve: Option<ResolveUnion>,
    pub levels: Option<HeightElement>,
    pub scale: Option<FontSizeUnion>,
    pub translate: Option<Translate>,
    pub zero: Option<TickExtraUnion>,
    pub counts: Option<TickExtraUnion>,
    pub generate: Option<TickExtraUnion>,
    pub keys: Option<GroupbyUnion>,
    pub padding: Option<TransformPadding>,
    pub radius: Option<ColorUnion>,
    pub round: Option<TickExtraUnion>,
    pub key: Option<ColorUnion>,
    #[serde(rename = "parentKey")]
    pub parent_key: Option<ColorUnion>,
    pub method: Option<BackgroundElement>,
    #[serde(rename = "nodeSize")]
    pub node_size: Option<CenterUnion>,
    pub separation: Option<TickExtraUnion>,
    #[serde(rename = "paddingBottom")]
    pub padding_bottom: Option<HeightElement>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<HeightElement>,
    #[serde(rename = "paddingLeft")]
    pub padding_left: Option<HeightElement>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<HeightElement>,
    #[serde(rename = "paddingRight")]
    pub padding_right: Option<HeightElement>,
    #[serde(rename = "paddingTop")]
    pub padding_top: Option<HeightElement>,
    pub ratio: Option<HeightElement>,
    pub anchor: Option<TransformAnchor>,
    #[serde(rename = "avoidBaseMark")]
    pub avoid_base_mark: Option<TickExtraUnion>,
    #[serde(rename = "avoidMarks")]
    pub avoid_marks: Option<AvoidMarks>,
    #[serde(rename = "lineAnchor")]
    pub line_anchor: Option<BackgroundElement>,
    #[serde(rename = "markIndex")]
    pub mark_index: Option<HeightElement>,
    pub order: Option<HeightElement>,
    pub params: Option<ParamsUnion>,
    pub cross: Option<TickExtraUnion>,
    pub drop: Option<TickExtraUnion>,
    pub ops: Option<Ops>,
    pub base: Option<HeightElement>,
    pub divide: Option<CenterUnion>,
    pub interval: Option<TickExtraUnion>,
    pub maxbins: Option<HeightElement>,
    pub minstep: Option<HeightElement>,
    pub name: Option<BackgroundElement>,
    pub span: Option<HeightElement>,
    pub steps: Option<StepsUnion>,
    pub case: Option<CaseUnion>,
    pub pattern: Option<BackgroundElement>,
    pub stopwords: Option<BackgroundElement>,
    pub distribution: Option<Distribution>,
    pub maxsteps: Option<HeightElement>,
    pub minsteps: Option<HeightElement>,
    pub expr: Option<String>,
    pub index: Option<BackgroundElement>,
    pub initonly: Option<TickExtraUnion>,
    pub keyvals: Option<ArrayOrSignal>,
    pub value: Option<serde_json::Value>,
    pub cumulative: Option<TickExtraUnion>,
    #[serde(rename = "default")]
    pub transform_default: Option<serde_json::Value>,
    pub from: Option<String>,
    pub limit: Option<HeightElement>,
    pub op: Option<TransformOp>,
    pub probs: Option<CenterUnion>,
    pub start: Option<HeightElement>,
    pub stop: Option<HeightElement>,
    pub timezone: Option<TimezoneUnion>,
    pub units: Option<Units>,
    pub frame: Option<Params>,
    #[serde(rename = "ignorePeers")]
    pub ignore_peers: Option<TickExtraUnion>,
    pub font: Option<ColorUnion>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<FontSizeUnion>,
    #[serde(rename = "fontSizeRange")]
    pub font_size_range: Option<CenterUnion>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<ColorUnion>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<ColorUnion>,
    pub rotate: Option<FontSizeUnion>,
    pub spiral: Option<BackgroundElement>,
    pub text: Option<ColorUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub function: Function,
    pub mean: Option<HeightElement>,
    pub stdev: Option<HeightElement>,
    pub max: Option<HeightElement>,
    pub min: Option<HeightElement>,
    pub bandwidth: Option<HeightElement>,
    pub field: Option<ColorUnion>,
    pub from: Option<String>,
    pub distributions: Option<ArrayOrSignal>,
    pub weights: Option<CenterUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontSizeSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub expr: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceElement {
    pub force: ForceEnum,
    pub x: Option<XUnion>,
    pub y: Option<XUnion>,
    pub iterations: Option<HeightElement>,
    pub radius: Option<FontSizeUnion>,
    pub strength: Option<StrengthUnion>,
    #[serde(rename = "distanceMax")]
    pub distance_max: Option<HeightElement>,
    #[serde(rename = "distanceMin")]
    pub distance_min: Option<HeightElement>,
    pub theta: Option<HeightElement>,
    pub distance: Option<FontSizeUnion>,
    pub id: Option<ColorUnion>,
    pub links: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrengthSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub expr: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickySignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaddingExpr {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub expr: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareClass {
    pub signal: Option<String>,
    pub field: Option<CompareField>,
    pub order: Option<OrderUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub signal_ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VegaEncode {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub align: Option<LayoutAlign>,
    pub bounds: Option<BoundsUnion>,
    pub center: Option<Center>,
    pub columns: Option<HeightElement>,
    #[serde(rename = "footerBand")]
    pub footer_band: Option<FooterBandUnion>,
    #[serde(rename = "headerBand")]
    pub header_band: Option<HeaderBandUnion>,
    pub offset: Option<LayoutOffset>,
    pub padding: Option<PaddingUnion>,
    #[serde(rename = "titleAnchor")]
    pub title_anchor: Option<TitleAnchor>,
    #[serde(rename = "titleBand")]
    pub title_band: Option<TitleBandUnion>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HilariousSignalRef {
    pub signal: Option<String>,
    pub column: Option<GridAlignUnion>,
    pub row: Option<GridAlignUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbitiousSignalRef {
    pub signal: Option<String>,
    pub column: Option<TickExtraUnion>,
    pub row: Option<TickExtraUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FooterBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<HeightElement>,
    pub row: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<HeightElement>,
    pub row: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CunningSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "columnFooter")]
    pub column_footer: Option<HeightElement>,
    #[serde(rename = "columnHeader")]
    pub column_header: Option<HeightElement>,
    #[serde(rename = "columnTitle")]
    pub column_title: Option<HeightElement>,
    #[serde(rename = "rowFooter")]
    pub row_footer: Option<HeightElement>,
    #[serde(rename = "rowHeader")]
    pub row_header: Option<HeightElement>,
    #[serde(rename = "rowTitle")]
    pub row_title: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagentaSignalRef {
    pub signal: Option<String>,
    pub column: Option<HeightElement>,
    pub row: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleAnchorSignalRef {
    pub signal: Option<String>,
    pub column: Option<PurpleColumn>,
    pub row: Option<PurpleColumn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<HeightElement>,
    pub row: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    pub aria: Option<bool>,
    #[serde(rename = "clipHeight")]
    pub clip_height: Option<HeightElement>,
    #[serde(rename = "columnPadding")]
    pub column_padding: Option<HeightElement>,
    pub columns: Option<HeightElement>,
    #[serde(rename = "cornerRadius")]
    pub corner_radius: Box<Option<Box<AngleUnion>>>,
    pub description: Option<String>,
    pub direction: Option<Direction>,
    pub encode: Option<LegendEncode>,
    pub fill: Option<String>,
    #[serde(rename = "fillColor")]
    pub fill_color: Option<ColorValue>,
    pub format: Option<LegendFormat>,
    #[serde(rename = "formatType")]
    pub format_type: Option<FormatTypeUnion>,
    #[serde(rename = "gradientLength")]
    pub gradient_length: Option<HeightElement>,
    #[serde(rename = "gradientOpacity")]
    pub gradient_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "gradientStrokeColor")]
    pub gradient_stroke_color: Option<ColorValue>,
    #[serde(rename = "gradientStrokeWidth")]
    pub gradient_stroke_width: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "gradientThickness")]
    pub gradient_thickness: Option<HeightElement>,
    #[serde(rename = "gridAlign")]
    pub grid_align: Option<GridAlignUnion>,
    #[serde(rename = "labelAlign")]
    pub label_align: Option<AlignUnion>,
    #[serde(rename = "labelBaseline")]
    pub label_baseline: Option<BaselineUnion>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<ColorValue>,
    #[serde(rename = "labelFont")]
    pub label_font: Option<FontUnion>,
    #[serde(rename = "labelFontSize")]
    pub label_font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelFontStyle")]
    pub label_font_style: Option<FontUnion>,
    #[serde(rename = "labelFontWeight")]
    pub label_font_weight: Option<FontWeightUnion>,
    #[serde(rename = "labelLimit")]
    pub label_limit: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOffset")]
    pub label_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOpacity")]
    pub label_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "labelOverlap")]
    pub label_overlap: Option<LabelOverlap>,
    #[serde(rename = "labelSeparation")]
    pub label_separation: Option<HeightElement>,
    #[serde(rename = "legendX")]
    pub legend_x: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "legendY")]
    pub legend_y: Box<Option<Box<AngleUnion>>>,
    pub offset: Box<Option<Box<AngleUnion>>>,
    pub opacity: Option<String>,
    pub orient: Option<LegendOrient>,
    pub padding: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "rowPadding")]
    pub row_padding: Option<HeightElement>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    #[serde(rename = "strokeColor")]
    pub stroke_color: Option<ColorValue>,
    #[serde(rename = "strokeDash")]
    pub stroke_dash: Option<String>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<String>,
    #[serde(rename = "symbolDash")]
    pub symbol_dash: Option<DomainDashUnion>,
    #[serde(rename = "symbolDashOffset")]
    pub symbol_dash_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "symbolFillColor")]
    pub symbol_fill_color: Option<ColorValue>,
    #[serde(rename = "symbolLimit")]
    pub symbol_limit: Option<HeightElement>,
    #[serde(rename = "symbolOffset")]
    pub symbol_offset: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "symbolOpacity")]
    pub symbol_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "symbolSize")]
    pub symbol_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "symbolStrokeColor")]
    pub symbol_stroke_color: Option<ColorValue>,
    #[serde(rename = "symbolStrokeWidth")]
    pub symbol_stroke_width: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "symbolType")]
    pub symbol_type: Option<FontUnion>,
    #[serde(rename = "tickCount")]
    pub tick_count: Option<TickCount>,
    #[serde(rename = "tickMinStep")]
    pub tick_min_step: Option<HeightElement>,
    pub title: Option<TextOrSignal>,
    #[serde(rename = "titleAlign")]
    pub title_align: Option<AlignUnion>,
    #[serde(rename = "titleAnchor")]
    pub title_anchor: Option<AnchorUnion>,
    #[serde(rename = "titleBaseline")]
    pub title_baseline: Option<BaselineUnion>,
    #[serde(rename = "titleColor")]
    pub title_color: Option<ColorValue>,
    #[serde(rename = "titleFont")]
    pub title_font: Option<FontUnion>,
    #[serde(rename = "titleFontSize")]
    pub title_font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleFontStyle")]
    pub title_font_style: Option<FontUnion>,
    #[serde(rename = "titleFontWeight")]
    pub title_font_weight: Option<FontWeightUnion>,
    #[serde(rename = "titleLimit")]
    pub title_limit: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleLineHeight")]
    pub title_line_height: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleOpacity")]
    pub title_opacity: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "titleOrient")]
    pub title_orient: Option<TitleOrientUnion>,
    #[serde(rename = "titlePadding")]
    pub title_padding: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "type")]
    pub legend_type: Option<LegendType>,
    pub values: Option<ArrayOrSignal>,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegendEncode {
    pub entries: Option<GuideEncode>,
    pub gradient: Option<GuideEncode>,
    pub labels: Option<GuideEncode>,
    pub legend: Option<GuideEncode>,
    pub symbols: Option<GuideEncode>,
    pub title: Option<GuideEncode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriskySignalRef {
    pub date: Option<String>,
    pub day: Option<String>,
    pub hours: Option<String>,
    pub milliseconds: Option<String>,
    pub minutes: Option<String>,
    pub month: Option<String>,
    pub quarter: Option<String>,
    pub seconds: Option<String>,
    pub week: Option<String>,
    pub year: Option<String>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleOrientElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientValue {
    pub scale: Box<Option<Box<Field>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<Field>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mark {
    pub from: Option<From>,
    #[serde(rename = "type")]
    pub mark_type: String,
    pub aria: Option<bool>,
    pub clip: Option<Markclip>,
    pub description: Option<String>,
    pub encode: Option<VegaEncode>,
    pub interactive: Option<TickExtraUnion>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub on: Option<Vec<OnMarkTrigger>>,
    pub role: Option<String>,
    pub sort: Option<Compare>,
    pub style: Option<Style>,
    pub transform: Option<Vec<TransformMark>>,
    pub axes: Option<Vec<Axis>>,
    pub data: Option<Vec<Data>>,
    pub layout: Option<Layout>,
    pub legends: Option<Vec<Legend>>,
    pub marks: Option<Vec<Mark>>,
    pub projections: Option<Vec<Projection>>,
    pub scales: Option<Vec<Scale>>,
    pub signals: Option<Vec<Signal>>,
    pub title: Option<Title>,
    pub usermeta: Option<HashMap<String, Option<serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkclipSignalRef {
    pub signal: Option<String>,
    pub path: Option<BackgroundElement>,
    pub sphere: Option<BackgroundElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct From {
    pub data: Option<String>,
    pub facet: Option<Facet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub data: String,
    pub field: Option<String>,
    pub name: String,
    pub aggregate: Option<Aggregate>,
    pub groupby: Option<Style>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aggregate {
    #[serde(rename = "as")]
    pub aggregate_as: Option<Vec<String>>,
    pub cross: Option<bool>,
    pub fields: Option<Vec<String>>,
    pub ops: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnMarkTrigger {
    pub modify: Option<String>,
    pub trigger: String,
    pub values: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Projection {
    pub center: Option<CenterUnion>,
    #[serde(rename = "clipAngle")]
    pub clip_angle: Option<HeightElement>,
    #[serde(rename = "clipExtent")]
    pub clip_extent: Option<Extent>,
    pub extent: Option<Extent>,
    pub fit: Option<Fit>,
    pub name: String,
    pub parallels: Option<CenterUnion>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<HeightElement>,
    pub precision: Option<HeightElement>,
    pub rotate: Option<CenterUnion>,
    pub scale: Option<HeightElement>,
    pub size: Option<CenterUnion>,
    pub translate: Option<CenterUnion>,
    #[serde(rename = "type")]
    pub projection_type: Option<BackgroundElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    pub domain: Option<ScaleData>,
    #[serde(rename = "domainMax")]
    pub domain_max: Option<HeightElement>,
    #[serde(rename = "domainMid")]
    pub domain_mid: Option<HeightElement>,
    #[serde(rename = "domainMin")]
    pub domain_min: Option<HeightElement>,
    #[serde(rename = "domainRaw")]
    pub domain_raw: Option<ArrayOrSignal>,
    pub name: String,
    pub nice: Option<PurpleBooleanOrSignal>,
    pub reverse: Option<TickExtraUnion>,
    pub round: Option<TickExtraUnion>,
    #[serde(rename = "type")]
    pub scale_type: Option<ScaleType>,
    #[serde(rename = "domainImplicit")]
    pub domain_implicit: Option<TickExtraUnion>,
    pub interpolate: Option<ScaleInterpolate>,
    pub range: Option<RangeUnion>,
    pub align: Option<HeightElement>,
    pub padding: Option<HeightElement>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<HeightElement>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<HeightElement>,
    pub zero: Option<TickExtraUnion>,
    pub bins: Option<ScaleBins>,
    pub clamp: Option<TickExtraUnion>,
    pub base: Option<HeightElement>,
    pub exponent: Option<HeightElement>,
    pub constant: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleBinsSignalRef {
    pub start: Option<HeightElement>,
    pub step: Option<HeightElement>,
    pub stop: Option<HeightElement>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MischievousSignalRef {
    pub data: Option<String>,
    pub field: Option<BackgroundElement>,
    pub sort: Option<TentacledSort>,
    pub fields: Option<Vec<PurpleStringOrSignal>>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BraggadociousSignalRef {
    pub signal: Option<String>,
    pub data: Option<String>,
    pub field: Option<BackgroundElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleSort {
    pub field: Option<BackgroundElement>,
    pub op: Option<BackgroundElement>,
    pub order: Option<SortSortOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleInterpolateSignalRef {
    pub signal: Option<String>,
    pub gamma: Option<HeightElement>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<BackgroundElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef1 {
    pub signal: Option<String>,
    pub interval: Option<Interval>,
    pub step: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef2 {
    pub count: Option<HeightElement>,
    pub extent: Option<CenterUnion>,
    pub scheme: Option<Scheme>,
    pub data: Option<String>,
    pub field: Option<BackgroundElement>,
    pub sort: Option<StickySort>,
    pub fields: Option<Vec<FluffyStringOrSignal>>,
    pub signal: Option<String>,
    pub step: Option<HeightElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef3 {
    pub signal: Option<String>,
    pub data: Option<String>,
    pub field: Option<BackgroundElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffySort {
    pub field: Option<BackgroundElement>,
    pub op: Option<BackgroundElement>,
    pub order: Option<SortSortOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signal {
    pub description: Option<String>,
    pub name: String,
    pub on: Option<Vec<OnEvent>>,
    pub push: Option<Push>,
    pub bind: Option<Bind>,
    pub react: Option<bool>,
    pub update: Option<String>,
    pub value: Option<serde_json::Value>,
    pub init: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bind {
    pub debounce: Option<f64>,
    pub element: Option<String>,
    pub input: Option<serde_json::Value>,
    pub name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub options: Option<Vec<Option<serde_json::Value>>>,
    pub max: Option<f64>,
    pub min: Option<f64>,
    pub step: Option<f64>,
    pub event: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnEvent {
    pub events: EventsUnion,
    pub force: Option<bool>,
    pub encode: Option<String>,
    pub update: Option<Update>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listener {
    pub signal: Option<String>,
    pub scale: Option<String>,
    pub between: Option<Vec<Stream>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub listener_type: Option<String>,
    pub stream: Box<Option<Stream>>,
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub between: Option<Vec<Stream>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub stream_type: Option<String>,
    pub stream: Box<Option<Stream>>,
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamClass {
    pub signal: Option<String>,
    pub scale: Option<String>,
    pub between: Option<Vec<Stream>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<String>,
    pub stream: Box<Option<Stream>>,
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expr {
    #[serde(rename = "as")]
    pub expr_as: Option<String>,
    pub expr: Option<String>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compare {
    pub field: Option<CompareField>,
    pub order: Option<OrderUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleClass {
    pub align: Option<AlignUnion>,
    pub anchor: Option<AnchorUnion>,
    pub angle: Box<Option<Box<AngleUnion>>>,
    pub aria: Option<bool>,
    pub baseline: Option<BaselineUnion>,
    pub color: Option<ColorValue>,
    pub dx: Box<Option<Box<AngleUnion>>>,
    pub dy: Box<Option<Box<AngleUnion>>>,
    pub encode: Option<TitleEncode>,
    pub font: Option<FontUnion>,
    #[serde(rename = "fontSize")]
    pub font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<FontUnion>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<FontWeightUnion>,
    pub frame: Option<FrameUnion>,
    pub interactive: Option<bool>,
    pub limit: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "lineHeight")]
    pub line_height: Box<Option<Box<AngleUnion>>>,
    pub name: Option<String>,
    pub offset: Box<Option<Box<AngleUnion>>>,
    pub orient: Option<TitleOrient>,
    pub style: Option<Style>,
    pub subtitle: Option<TextOrSignal>,
    #[serde(rename = "subtitleColor")]
    pub subtitle_color: Option<ColorValue>,
    #[serde(rename = "subtitleFont")]
    pub subtitle_font: Option<FontUnion>,
    #[serde(rename = "subtitleFontSize")]
    pub subtitle_font_size: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "subtitleFontStyle")]
    pub subtitle_font_style: Option<FontUnion>,
    #[serde(rename = "subtitleFontWeight")]
    pub subtitle_font_weight: Option<FontWeightUnion>,
    #[serde(rename = "subtitleLineHeight")]
    pub subtitle_line_height: Box<Option<Box<AngleUnion>>>,
    #[serde(rename = "subtitlePadding")]
    pub subtitle_padding: Option<HeightElement>,
    pub text: Option<TextOrSignal>,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleEncode {
    pub group: Option<GuideEncode>,
    pub subtitle: Option<GuideEncode>,
    pub title: Option<GuideEncode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformMark {
    pub fields: Option<FieldsUnion>,
    pub query: Option<ArrayOrSignal>,
    pub signal: Option<String>,
    #[serde(rename = "type")]
    pub transform_mark_type: TransformMarkType,
    pub filter: Option<serde_json::Value>,
    pub ignore: Option<HeightElement>,
    #[serde(rename = "as")]
    pub transform_mark_as: Option<AsUnion>,
    pub orient: Option<TransformOrient>,
    pub require: Option<FormatTypeSignalRef>,
    pub shape: Option<ShapeUnion>,
    #[serde(rename = "sourceX")]
    pub source_x: Option<ColorUnion>,
    #[serde(rename = "sourceY")]
    pub source_y: Option<ColorUnion>,
    #[serde(rename = "targetX")]
    pub target_x: Option<ColorUnion>,
    #[serde(rename = "targetY")]
    pub target_y: Option<ColorUnion>,
    #[serde(rename = "endAngle")]
    pub end_angle: Option<HeightElement>,
    pub field: Option<ColorUnion>,
    pub sort: Option<SortUnion>,
    #[serde(rename = "startAngle")]
    pub start_angle: Option<HeightElement>,
    pub groupby: Option<GroupbyUnion>,
    pub offset: Option<TransformOffset>,
    pub alpha: Option<HeightElement>,
    #[serde(rename = "alphaMin")]
    pub alpha_min: Option<HeightElement>,
    #[serde(rename = "alphaTarget")]
    pub alpha_target: Option<HeightElement>,
    pub forces: Option<Vec<ForceElement>>,
    pub iterations: Option<HeightElement>,
    pub restart: Option<TickExtraUnion>,
    #[serde(rename = "static")]
    pub transform_mark_static: Option<TickExtraUnion>,
    #[serde(rename = "velocityDecay")]
    pub velocity_decay: Option<HeightElement>,
    pub geojson: Option<ColorUnion>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<FontSizeUnion>,
    pub projection: Option<String>,
    pub color: Option<ColorUnion>,
    pub opacity: Option<FontSizeUnion>,
    pub resolve: Option<ResolveUnion>,
    pub padding: Option<TransformPadding>,
    pub radius: Option<ColorUnion>,
    pub size: Option<StepsUnion>,
    pub round: Option<TickExtraUnion>,
    pub key: Option<ColorUnion>,
    #[serde(rename = "parentKey")]
    pub parent_key: Option<ColorUnion>,
    pub method: Option<BackgroundElement>,
    #[serde(rename = "nodeSize")]
    pub node_size: Option<CenterUnion>,
    pub separation: Option<TickExtraUnion>,
    #[serde(rename = "paddingBottom")]
    pub padding_bottom: Option<HeightElement>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<HeightElement>,
    #[serde(rename = "paddingLeft")]
    pub padding_left: Option<HeightElement>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<HeightElement>,
    #[serde(rename = "paddingRight")]
    pub padding_right: Option<HeightElement>,
    #[serde(rename = "paddingTop")]
    pub padding_top: Option<HeightElement>,
    pub ratio: Option<HeightElement>,
    pub anchor: Option<TransformAnchor>,
    #[serde(rename = "avoidBaseMark")]
    pub avoid_base_mark: Option<TickExtraUnion>,
    #[serde(rename = "avoidMarks")]
    pub avoid_marks: Option<AvoidMarks>,
    #[serde(rename = "lineAnchor")]
    pub line_anchor: Option<BackgroundElement>,
    #[serde(rename = "markIndex")]
    pub mark_index: Option<HeightElement>,
    pub base: Option<HeightElement>,
    pub divide: Option<CenterUnion>,
    pub extent: Option<ArrayOrSignal>,
    pub interval: Option<TickExtraUnion>,
    pub maxbins: Option<HeightElement>,
    pub minstep: Option<HeightElement>,
    pub name: Option<BackgroundElement>,
    pub nice: Option<TickExtraUnion>,
    pub span: Option<HeightElement>,
    pub step: Option<HeightElement>,
    pub steps: Option<CenterUnion>,
    pub smooth: Option<TickExtraUnion>,
    pub expr: Option<String>,
    pub initonly: Option<TickExtraUnion>,
    pub ops: Option<Ops>,
    #[serde(rename = "default")]
    pub transform_mark_default: Option<serde_json::Value>,
    pub from: Option<String>,
    pub values: Option<GroupbyUnion>,
    pub timezone: Option<TimezoneUnion>,
    pub units: Option<Units>,
    pub frame: Option<Params>,
    #[serde(rename = "ignorePeers")]
    pub ignore_peers: Option<TickExtraUnion>,
    pub params: Option<Params>,
    pub x: Option<ColorUnion>,
    pub y: Option<ColorUnion>,
    pub font: Option<ColorUnion>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<FontSizeUnion>,
    #[serde(rename = "fontSizeRange")]
    pub font_size_range: Option<CenterUnion>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<ColorUnion>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<ColorUnion>,
    pub rotate: Option<FontSizeUnion>,
    pub spiral: Option<BackgroundElement>,
    pub text: Option<ColorUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef4 {
    pub bottom: Option<f64>,
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Autosize {
    AutosizeSignalRef(AutosizeSignalRef),
    Enum(AutosizeEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AngleUnion {
    AngleClass(AngleClass),
    AngleElementArray(Vec<AngleElement>),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Band {
    Bool(bool),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontUnion {
    FontElementArray(Vec<FontElement>),
    String(String),
    StringValue(StringValue),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorValue {
    BaseColorValueArray(Vec<BaseColorValue>),
    PurpleBaseColorValue(PurpleBaseColorValue),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValue {
    AngleClass(AngleClass),
    AngleElementArray(Vec<AngleElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainDashUnion {
    ArrayValue(ArrayValue),
    UnionArray(Vec<Dash>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dash {
    DomainDashClass(DomainDashClass),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Style {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AxeFormat {
    FluffySignalRef(FluffySignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatTypeUnion {
    Enum(FormatTypeEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlignUnion {
    AlignElementArray(Vec<AlignElement>),
    AlignValue(AlignValue),
    Enum(AlignEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BaselineUnion {
    BaselineElementArray(Vec<BaselineElement>),
    BaselineValue(BaselineValue),
    Enum(Baseline),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelBound {
    Bool(bool),
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeightElement {
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontWeightUnion {
    Double(f64),
    Enum(FontWeight),
    FontWeightElementArray(Vec<FontWeightElement>),
    FontWeightValue(FontWeightValue),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelOverlap {
    Bool(bool),
    Enum(LabelOverlapEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AxeOrient {
    Enum(TitleOrientEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickBand {
    Enum(TickBandEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickCount {
    Double(f64),
    Enum(TickCountEnum),
    TickCountSignalRef(TickCountSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Interval {
    Enum(TickCountEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickExtraUnion {
    Bool(bool),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickRoundUnion {
    Bool(bool),
    BooleanValue(BooleanValue),
    TickRoundElementArray(Vec<TickRoundElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextOrSignal {
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnchorUnion {
    AnchorElementArray(Vec<AnchorElement>),
    AnchorValue(AnchorValue),
    Enum(Anchor),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArrayOrSignal {
    AnythingArray(Vec<Option<serde_json::Value>>),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BackgroundElement {
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParseUnion {
    Enum(ParseEnum),
    ParseSignalRef(ParseSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Remove {
    Bool(bool),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformAnchor {
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<SchemeElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemeElement {
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AvoidMarks {
    FormatTypeSignalRef(FormatTypeSignalRef),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepsUnion {
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterElement {
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaseUnion {
    Enum(CaseEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorUnion {
    ColorSignalRef(ColorSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterUnion {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldsUnion {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<Option<FieldsField>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldsField {
    String(String),
    TentacledSignalRef(TentacledSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontSizeUnion {
    Double(f64),
    FontSizeSignalRef(FontSizeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrengthUnion {
    Double(f64),
    StrengthSignalRef(StrengthSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum XUnion {
    Double(f64),
    StickySignalRef(StickySignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<Option<HeightElement>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupbyUnion {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<GroupbyElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupbyElement {
    ColorSignalRef(ColorSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformOffset {
    Enum(OffsetEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformOp {
    Enum(PurpleOp),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ops {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<OpElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpElement {
    Enum(FluffyOp),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformOrient {
    Enum(PurpleOrient),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformPadding {
    Double(f64),
    PaddingExpr(PaddingExpr),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamsUnion {
    Bool(bool),
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<Option<HeightElement>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResolveUnion {
    Enum(ResolveEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShapeUnion {
    Enum(ShapeEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortUnion {
    Bool(bool),
    CompareClass(CompareClass),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompareField {
    IndigoSignalRef(IndigoSignalRef),
    String(String),
    UnionArray(Vec<FieldField>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldField {
    IndigoSignalRef(IndigoSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderUnion {
    Enum(OrderEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<SortOrderElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortOrderElement {
    Enum(OrderEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimezoneUnion {
    Enum(TimezoneEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AsUnion {
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
    UnionArray(Vec<Option<BackgroundElement>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Translate {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<TranslateElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslateElement {
    Double(f64),
    FontSizeSignalRef(FontSizeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Units {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<UnitElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnitElement {
    Enum(UnitEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValuesUnion {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<ValueElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueElement {
    Double(f64),
    IndecentSignalRef(IndecentSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutAlign {
    Enum(GridAlignEnum),
    HilariousSignalRef(HilariousSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridAlignUnion {
    Enum(GridAlignEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoundsUnion {
    Enum(BoundsEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Center {
    AmbitiousSignalRef(AmbitiousSignalRef),
    Bool(bool),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FooterBandUnion {
    Double(f64),
    FooterBandSignalRef(FooterBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeaderBandUnion {
    Double(f64),
    HeaderBandSignalRef(HeaderBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutOffset {
    CunningSignalRef(CunningSignalRef),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaddingUnion {
    Double(f64),
    MagentaSignalRef(MagentaSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleAnchor {
    Enum(TitleAnchorEnum),
    TitleAnchorSignalRef(TitleAnchorSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleColumn {
    Enum(TitleAnchorEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleBandUnion {
    Double(f64),
    TitleBandSignalRef(TitleBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegendFormat {
    FriskySignalRef(FriskySignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegendOrient {
    Enum(FluffyOrient),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleOrientUnion {
    Enum(TitleOrientEnum),
    OrientValue(OrientValue),
    TitleOrientElementArray(Vec<TitleOrientElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Markclip {
    Bool(bool),
    MarkclipSignalRef(MarkclipSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Extent {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<ClipExtentElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClipExtentElement {
    FormatTypeSignalRef(FormatTypeSignalRef),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Fit {
    AnythingArray(Vec<Option<serde_json::Value>>),
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleBins {
    ScaleBinsSignalRef(ScaleBinsSignalRef),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleData {
    MischievousSignalRef(MischievousSignalRef),
    UnionArray(Vec<Option<Domain>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Domain {
    Bool(bool),
    Double(f64),
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
    UnionArray(Vec<CenterElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleStringOrSignal {
    BraggadociousSignalRef(BraggadociousSignalRef),
    String(String),
    UnionArray(Vec<FieldElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldElement {
    Bool(bool),
    Double(f64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledSort {
    Bool(bool),
    PurpleSort(PurpleSort),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortSortOrder {
    Enum(OrderEnum),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleInterpolate {
    ScaleInterpolateSignalRef(ScaleInterpolateSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleBooleanOrSignal {
    Bool(bool),
    Double(f64),
    Enum(TickCountEnum),
    SignalRef1(SignalRef1),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeUnion {
    Enum(RangeEnum),
    SignalRef2(SignalRef2),
    UnionArray(Vec<Option<Domain>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyStringOrSignal {
    SignalRef3(SignalRef3),
    String(String),
    UnionArray(Vec<FieldElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Scheme {
    FormatTypeSignalRef(FormatTypeSignalRef),
    String(String),
    UnionArray(Vec<SchemeElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickySort {
    Bool(bool),
    FluffySort(FluffySort),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventsUnion {
    ListenerArray(Vec<Listener>),
    StreamClass(StreamClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Update {
    Expr(Expr),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Title {
    String(String),
    TitleClass(TitleClass),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FrameUnion {
    Enum(Frame),
    FontElementArray(Vec<FontElement>),
    StringValue(StringValue),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleOrient {
    Enum(TentacledOrient),
    FormatTypeSignalRef(FormatTypeSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Padding {
    Double(f64),
    SignalRef4(SignalRef4),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Contains {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "padding")]
    Padding,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AutosizeEnum {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "fit-x")]
    FitX,
    #[serde(rename = "fit-y")]
    FitY,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pad")]
    Pad,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormatTypeEnum {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "utc")]
    Utc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlignEnum {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Baseline {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "line-bottom")]
    LineBottom,
    #[serde(rename = "line-top")]
    LineTop,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FontWeight {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "bolder")]
    Bolder,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "100")]
    The100,
    #[serde(rename = "200")]
    The200,
    #[serde(rename = "300")]
    The300,
    #[serde(rename = "400")]
    The400,
    #[serde(rename = "500")]
    The500,
    #[serde(rename = "600")]
    The600,
    #[serde(rename = "700")]
    The700,
    #[serde(rename = "800")]
    The800,
    #[serde(rename = "900")]
    The900,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LabelOverlapEnum {
    #[serde(rename = "greedy")]
    Greedy,
    #[serde(rename = "parity")]
    Parity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TitleOrientEnum {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TickBandEnum {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "extent")]
    Extent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TickCountEnum {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "millisecond")]
    Millisecond,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Anchor {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "start")]
    Start,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
    #[serde(rename = "exterior")]
    Exterior,
    #[serde(rename = "interior")]
    Interior,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParseEnum {
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CaseEnum {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "upper")]
    Upper,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Function {
    #[serde(rename = "kde")]
    Kde,
    #[serde(rename = "lognormal")]
    Lognormal,
    #[serde(rename = "mixture")]
    Mixture,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "uniform")]
    Uniform,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ForceEnum {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "collide")]
    Collide,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "nbody")]
    Nbody,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OffsetEnum {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "normalize")]
    Normalize,
    #[serde(rename = "zero")]
    Zero,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleOp {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "__count__")]
    OpCount,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FluffyOp {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "cume_dist")]
    CumeDist,
    #[serde(rename = "dense_rank")]
    DenseRank,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "first_value")]
    FirstValue,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "last_value")]
    LastValue,
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "next_value")]
    NextValue,
    #[serde(rename = "nth_value")]
    NthValue,
    #[serde(rename = "ntile")]
    Ntile,
    #[serde(rename = "__count__")]
    OpCount,
    #[serde(rename = "percent_rank")]
    PercentRank,
    #[serde(rename = "prev_value")]
    PrevValue,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "rank")]
    Rank,
    #[serde(rename = "row_number")]
    RowNumber,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleOrient {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "radial")]
    Radial,
    #[serde(rename = "vertical")]
    Vertical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolveEnum {
    #[serde(rename = "independent")]
    Independent,
    #[serde(rename = "shared")]
    Shared,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShapeEnum {
    #[serde(rename = "arc")]
    Arc,
    #[serde(rename = "curve")]
    Curve,
    #[serde(rename = "diagonal")]
    Diagonal,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "orthogonal")]
    Orthogonal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderEnum {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimezoneEnum {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "utc")]
    Utc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransformType {
    #[serde(rename = "aggregate")]
    Aggregate,
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "collect")]
    Collect,
    #[serde(rename = "contour")]
    Contour,
    #[serde(rename = "countpattern")]
    Countpattern,
    #[serde(rename = "cross")]
    Cross,
    #[serde(rename = "crossfilter")]
    Crossfilter,
    #[serde(rename = "density")]
    Density,
    #[serde(rename = "dotbin")]
    Dotbin,
    #[serde(rename = "extent")]
    Extent,
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "flatten")]
    Flatten,
    #[serde(rename = "fold")]
    Fold,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "formula")]
    Formula,
    #[serde(rename = "geojson")]
    Geojson,
    #[serde(rename = "geopath")]
    Geopath,
    #[serde(rename = "geopoint")]
    Geopoint,
    #[serde(rename = "geoshape")]
    Geoshape,
    #[serde(rename = "graticule")]
    Graticule,
    #[serde(rename = "heatmap")]
    Heatmap,
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "impute")]
    Impute,
    #[serde(rename = "isocontour")]
    Isocontour,
    #[serde(rename = "joinaggregate")]
    Joinaggregate,
    #[serde(rename = "kde")]
    Kde,
    #[serde(rename = "kde2d")]
    Kde2D,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "linkpath")]
    Linkpath,
    #[serde(rename = "loess")]
    Loess,
    #[serde(rename = "lookup")]
    Lookup,
    #[serde(rename = "nest")]
    Nest,
    #[serde(rename = "pack")]
    Pack,
    #[serde(rename = "partition")]
    Partition,
    #[serde(rename = "pie")]
    Pie,
    #[serde(rename = "pivot")]
    Pivot,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "quantile")]
    Quantile,
    #[serde(rename = "regression")]
    Regression,
    #[serde(rename = "resolvefilter")]
    Resolvefilter,
    #[serde(rename = "sample")]
    Sample,
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "stratify")]
    Stratify,
    #[serde(rename = "timeunit")]
    Timeunit,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "treelinks")]
    Treelinks,
    #[serde(rename = "treemap")]
    Treemap,
    #[serde(rename = "voronoi")]
    Voronoi,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "wordcloud")]
    Wordcloud,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnitEnum {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "dayofyear")]
    Dayofyear,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GridAlignEnum {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "each")]
    Each,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BoundsEnum {
    #[serde(rename = "flush")]
    Flush,
    #[serde(rename = "full")]
    Full,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TitleAnchorEnum {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "start")]
    Start,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LegendType {
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "symbol")]
    Symbol,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FluffyOrient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RangeEnum {
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "diverging")]
    Diverging,
    #[serde(rename = "heatmap")]
    Heatmap,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "ramp")]
    Ramp,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "width")]
    Width,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScaleType {
    #[serde(rename = "band")]
    Band,
    #[serde(rename = "bin-ordinal")]
    BinOrdinal,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "pow")]
    Pow,
    #[serde(rename = "quantile")]
    Quantile,
    #[serde(rename = "quantize")]
    Quantize,
    #[serde(rename = "sequential")]
    Sequential,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "symlog")]
    Symlog,
    #[serde(rename = "threshold")]
    Threshold,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "utc")]
    Utc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Push {
    #[serde(rename = "outer")]
    Outer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Frame {
    #[serde(rename = "bounds")]
    Bounds,
    #[serde(rename = "group")]
    Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TentacledOrient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransformMarkType {
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "collect")]
    Collect,
    #[serde(rename = "crossfilter")]
    Crossfilter,
    #[serde(rename = "dotbin")]
    Dotbin,
    #[serde(rename = "extent")]
    Extent,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "formula")]
    Formula,
    #[serde(rename = "geojson")]
    Geojson,
    #[serde(rename = "geopath")]
    Geopath,
    #[serde(rename = "geopoint")]
    Geopoint,
    #[serde(rename = "geoshape")]
    Geoshape,
    #[serde(rename = "heatmap")]
    Heatmap,
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "joinaggregate")]
    Joinaggregate,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "linkpath")]
    Linkpath,
    #[serde(rename = "lookup")]
    Lookup,
    #[serde(rename = "pack")]
    Pack,
    #[serde(rename = "partition")]
    Partition,
    #[serde(rename = "pie")]
    Pie,
    #[serde(rename = "resolvefilter")]
    Resolvefilter,
    #[serde(rename = "sample")]
    Sample,
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "stratify")]
    Stratify,
    #[serde(rename = "timeunit")]
    Timeunit,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "treemap")]
    Treemap,
    #[serde(rename = "voronoi")]
    Voronoi,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "wordcloud")]
    Wordcloud,
}
