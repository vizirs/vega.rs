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

extern crate serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vega {
    pub axes: Option<Vec<AxeElement>>,
    pub data: Option<Vec<InputDataSetDefinition>>,
    pub encode: Option<VegaEncode>,
    pub layout: Option<LayoutClass>,
    pub legends: Option<Vec<LegendElement>>,
    pub marks: Option<Vec<Mark>>,
    pub projections: Option<Vec<ProjectionElement>>,
    pub scales: Option<Vec<ScaleMapping>>,
    pub signals: Option<Vec<SignalElement>>,
    pub title: Option<Title>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub autosize: Option<Autosize>,
    pub background: Option<String>,
    pub config: Option<HashMap<String, Option<serde_json::Value>>>,
    pub description: Option<String>,
    pub height: Option<f64>,
    pub padding: Option<VegaPadding>,
    pub usermeta: Option<HashMap<String, Option<serde_json::Value>>>,
    pub width: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutosizeClass {
    pub contains: Option<Contains>,
    pub resize: Option<bool>,
    #[serde(rename = "type")]
    pub v4_type: AutosizeEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AxeElement {
    #[serde(rename = "bandPosition")]
    pub band_position: Box<Option<Box<ChiangMaiGoose>>>,
    pub domain: Option<bool>,
    #[serde(rename = "domainColor")]
    pub domain_color: Option<ColorRef>,
    #[serde(rename = "domainOpacity")]
    pub domain_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "domainWidth")]
    pub domain_width: Box<Option<Box<ChiangMaiGoose>>>,
    pub encode: Option<AxeEncode>,
    pub format: Option<PurpleFormat>,
    pub grid: Option<bool>,
    #[serde(rename = "gridColor")]
    pub grid_color: Option<ColorRef>,
    #[serde(rename = "gridDash")]
    pub grid_dash: Option<AxeGridDash>,
    #[serde(rename = "gridOpacity")]
    pub grid_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "gridScale")]
    pub grid_scale: Option<String>,
    #[serde(rename = "gridWidth")]
    pub grid_width: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelAlign")]
    pub label_align: Option<FluffyAlign>,
    #[serde(rename = "labelAngle")]
    pub label_angle: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelBaseline")]
    pub label_baseline: Option<FluffyBaseline>,
    #[serde(rename = "labelBound")]
    pub label_bound: Option<LabelBound>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<ColorRef>,
    #[serde(rename = "labelFlush")]
    pub label_flush: Option<LabelBound>,
    #[serde(rename = "labelFlushOffset")]
    pub label_flush_offset: Option<PortoCamel>,
    #[serde(rename = "labelFont")]
    pub label_font: Option<LabelFont>,
    #[serde(rename = "labelFontSize")]
    pub label_font_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelFontWeight")]
    pub label_font_weight: Option<FluffyFontWeight>,
    #[serde(rename = "labelLimit")]
    pub label_limit: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelOpacity")]
    pub label_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelOverlap")]
    pub label_overlap: Option<LabelOverlap>,
    #[serde(rename = "labelPadding")]
    pub label_padding: Box<Option<Box<ChiangMaiGoose>>>,
    pub labels: Option<bool>,
    #[serde(rename = "maxExtent")]
    pub max_extent: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "minExtent")]
    pub min_extent: Box<Option<Box<ChiangMaiGoose>>>,
    pub offset: Box<Option<Box<ChiangMaiGoose>>>,
    pub orient: AxeOrient,
    pub position: Box<Option<Box<ChiangMaiGoose>>>,
    pub scale: String,
    #[serde(rename = "tickColor")]
    pub tick_color: Option<ColorRef>,
    #[serde(rename = "tickCount")]
    pub tick_count: Option<TickCount>,
    #[serde(rename = "tickExtra")]
    pub tick_extra: Option<PurpleTartuGecko>,
    #[serde(rename = "tickOffset")]
    pub tick_offset: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "tickOpacity")]
    pub tick_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "tickRound")]
    pub tick_round: Option<TickRound>,
    pub ticks: Option<bool>,
    #[serde(rename = "tickSize")]
    pub tick_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "tickWidth")]
    pub tick_width: Box<Option<Box<ChiangMaiGoose>>>,
    pub title: Option<PurpleFormat>,
    #[serde(rename = "titleAlign")]
    pub title_align: Option<FluffyAlign>,
    #[serde(rename = "titleAngle")]
    pub title_angle: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleBaseline")]
    pub title_baseline: Option<FluffyBaseline>,
    #[serde(rename = "titleColor")]
    pub title_color: Option<ColorRef>,
    #[serde(rename = "titleFont")]
    pub title_font: Option<LabelFont>,
    #[serde(rename = "titleFontSize")]
    pub title_font_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleFontWeight")]
    pub title_font_weight: Option<FluffyFontWeight>,
    #[serde(rename = "titleLimit")]
    pub title_limit: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleOpacity")]
    pub title_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titlePadding")]
    pub title_padding: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleX")]
    pub title_x: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleY")]
    pub title_y: Box<Option<Box<ChiangMaiGoose>>>,
    pub values: Option<Values>,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarcelonaHawaiianMonkSeal {
    pub band: Option<Band>,
    pub exponent: Box<Option<Box<ChiangMaiGoose>>>,
    pub extra: Option<bool>,
    pub mult: Box<Option<Box<ChiangMaiGoose>>>,
    pub offset: Box<Option<Box<ChiangMaiGoose>>>,
    pub round: Option<bool>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasseTerreElephant {
    pub test: Option<String>,
    pub band: Option<Band>,
    pub exponent: Box<Option<Box<ChiangMaiGoose>>>,
    pub extra: Option<bool>,
    pub mult: Box<Option<Box<ChiangMaiGoose>>>,
    pub offset: Box<Option<Box<ChiangMaiGoose>>>,
    pub round: Option<bool>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldRefSignalRef {
    pub signal: Option<String>,
    pub datum: Box<Option<Box<FieldRef>>>,
    pub group: Box<Option<Box<FieldRef>>>,
    pub level: Option<f64>,
    pub parent: Box<Option<Box<FieldRef>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffySignalRef {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorClass {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
    pub gradient: Box<Option<Box<FieldRef>>>,
    pub color: Option<Color>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub b: Option<PurpleV4>,
    pub g: Option<PurpleV4>,
    pub r: Option<PurpleV4>,
    pub h: Option<PurpleV4>,
    pub l: Option<PurpleV4>,
    pub s: Option<PurpleV4>,
    pub a: Option<PurpleV4>,
    pub c: Option<PurpleV4>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AxeEncode {
    pub axis: Option<Axis>,
    pub domain: Option<Axis>,
    pub grid: Option<Axis>,
    pub labels: Option<Axis>,
    pub ticks: Option<Axis>,
    pub title: Option<Axis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Axis {
    pub interactive: Option<bool>,
    pub name: Option<String>,
    pub style: Option<Style>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleSignalRef {
    pub signal: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledSignalRef {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickySignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoSignalRef {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentSignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HilariousSignalRef {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbitiousSignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CunningSignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagentaSignalRef {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriskySignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickCountSignalRef {
    pub interval: Option<IntervalUnion>,
    pub step: Option<PortoCamel>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickRoundElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MischievousSignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDataSetDefinition {
    pub name: String,
    pub on: Option<Vec<InputDataSetDefinitionOn>>,
    pub transform: Option<Vec<InputDataSetDefinitionTransform>>,
    pub source: Option<Style>,
    pub format: Option<FormatClass>,
    pub values: Option<Vec<Option<serde_json::Value>>>,
    pub url: Option<PurpleFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatClass {
    pub copy: Option<bool>,
    pub parse: Option<ParseUnion>,
    pub property: Option<String>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<FormatType>,
    pub delimiter: Option<String>,
    pub feature: Option<String>,
    pub mesh: Option<String>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDataSetDefinitionOn {
    pub insert: Option<String>,
    pub modify: Option<String>,
    pub remove: Option<Remove>,
    pub toggle: Option<String>,
    pub trigger: String,
    pub values: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDataSetDefinitionTransform {
    #[serde(rename = "as")]
    pub v4_as: Option<As>,
    pub cross: Option<PurpleTartuGecko>,
    pub drop: Option<PurpleTartuGecko>,
    pub fields: Option<FieldsUnion>,
    pub groupby: Option<GroupbyUnion>,
    pub key: Option<PurpleHammerfestPonies>,
    pub ops: Option<Ops>,
    pub signal: Option<String>,
    #[serde(rename = "type")]
    pub v4_type: TransformType,
    pub anchor: Option<PortoCamel>,
    pub base: Option<PortoCamel>,
    pub divide: Option<CenterUnion>,
    pub extent: Option<Values>,
    pub field: Option<PurpleHammerfestPonies>,
    pub maxbins: Option<PortoCamel>,
    pub minstep: Option<PortoCamel>,
    pub name: Option<PurpleFormat>,
    pub nice: Option<PurpleTartuGecko>,
    pub step: Option<StepUnion>,
    pub steps: Option<StepUnion>,
    pub sort: Option<FluffyTartuGecko>,
    pub case: Option<CaseUnion>,
    pub pattern: Option<PurpleFormat>,
    pub stopwords: Option<PurpleFormat>,
    pub filter: Option<serde_json::Value>,
    pub distribution: Option<Distribution>,
    pub method: Option<PurpleFormat>,
    pub expr: Option<String>,
    pub initonly: Option<PurpleTartuGecko>,
    pub keyvals: Option<Values>,
    pub value: Option<serde_json::Value>,
    #[serde(rename = "default")]
    pub v4_default: Option<serde_json::Value>,
    pub from: Option<String>,
    pub values: Option<PurpleFields>,
    pub limit: Option<PortoCamel>,
    pub op: Option<TransformOp>,
    pub size: Option<StepUnion>,
    pub start: Option<PortoCamel>,
    pub stop: Option<PortoCamel>,
    pub frame: Option<ParamsUnion>,
    #[serde(rename = "ignorePeers")]
    pub ignore_peers: Option<PurpleTartuGecko>,
    pub params: Option<ParamsUnion>,
    pub orient: Option<OrientUnion>,
    pub require: Option<PurpleSignalRef>,
    pub shape: Option<ShapeUnion>,
    #[serde(rename = "sourceX")]
    pub source_x: Option<PurpleHammerfestPonies>,
    #[serde(rename = "sourceY")]
    pub source_y: Option<PurpleHammerfestPonies>,
    #[serde(rename = "targetX")]
    pub target_x: Option<PurpleHammerfestPonies>,
    #[serde(rename = "targetY")]
    pub target_y: Option<PurpleHammerfestPonies>,
    #[serde(rename = "endAngle")]
    pub end_angle: Option<PortoCamel>,
    #[serde(rename = "startAngle")]
    pub start_angle: Option<PortoCamel>,
    pub offset: Option<TransformOffset>,
    pub bandwidth: Option<PortoCamel>,
    #[serde(rename = "cellSize")]
    pub cell_size: Option<PortoCamel>,
    pub count: Option<PortoCamel>,
    pub smooth: Option<PurpleTartuGecko>,
    pub thresholds: Option<CenterUnion>,
    pub weight: Option<PurpleHammerfestPonies>,
    pub x: Option<PurpleHammerfestPonies>,
    pub y: Option<PurpleHammerfestPonies>,
    pub geojson: Option<PurpleHammerfestPonies>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<PointRadius>,
    pub projection: Option<String>,
    #[serde(rename = "extentMajor")]
    pub extent_major: Option<Values>,
    #[serde(rename = "extentMinor")]
    pub extent_minor: Option<Values>,
    pub precision: Option<PortoCamel>,
    #[serde(rename = "stepMajor")]
    pub step_major: Option<CenterUnion>,
    #[serde(rename = "stepMinor")]
    pub step_minor: Option<CenterUnion>,
    pub alpha: Option<PortoCamel>,
    #[serde(rename = "alphaMin")]
    pub alpha_min: Option<PortoCamel>,
    #[serde(rename = "alphaTarget")]
    pub alpha_target: Option<PortoCamel>,
    pub forces: Option<Vec<ForceElement>>,
    pub iterations: Option<PortoCamel>,
    pub restart: Option<PurpleTartuGecko>,
    #[serde(rename = "static")]
    pub v4_static: Option<PurpleTartuGecko>,
    #[serde(rename = "velocityDecay")]
    pub velocity_decay: Option<PortoCamel>,
    pub generate: Option<PurpleTartuGecko>,
    pub keys: Option<GroupbyUnion>,
    pub padding: Option<FluffyPuneHedgehog>,
    pub radius: Option<PurpleHammerfestPonies>,
    pub round: Option<PurpleTartuGecko>,
    #[serde(rename = "parentKey")]
    pub parent_key: Option<PurpleHammerfestPonies>,
    #[serde(rename = "nodeSize")]
    pub node_size: Option<CenterUnion>,
    pub separation: Option<PurpleTartuGecko>,
    #[serde(rename = "paddingBottom")]
    pub padding_bottom: Option<PortoCamel>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<PortoCamel>,
    #[serde(rename = "paddingLeft")]
    pub padding_left: Option<PortoCamel>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<PortoCamel>,
    #[serde(rename = "paddingRight")]
    pub padding_right: Option<PortoCamel>,
    #[serde(rename = "paddingTop")]
    pub padding_top: Option<PortoCamel>,
    pub ratio: Option<PortoCamel>,
    pub font: Option<PurpleHammerfestPonies>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<PointRadius>,
    #[serde(rename = "fontSizeRange")]
    pub font_size_range: Option<CenterUnion>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<PurpleHammerfestPonies>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<PurpleHammerfestPonies>,
    pub rotate: Option<PointRadius>,
    pub spiral: Option<PurpleFormat>,
    pub text: Option<PurpleHammerfestPonies>,
    pub query: Option<Values>,
    pub ignore: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub function: Function,
    pub mean: Option<PortoCamel>,
    pub stdev: Option<PortoCamel>,
    pub max: Option<PortoCamel>,
    pub min: Option<PortoCamel>,
    pub bandwidth: Option<PortoCamel>,
    pub field: Option<PurpleHammerfestPonies>,
    pub from: Option<String>,
    pub distributions: Option<Values>,
    pub weights: Option<CenterUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSignalRefClass {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub expr: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceElement {
    pub force: ForceEnum,
    pub x: Option<TentacledPuneHedgehog>,
    pub y: Option<TentacledPuneHedgehog>,
    pub iterations: Option<PortoCamel>,
    pub radius: Option<PointRadius>,
    pub strength: Option<FluffyPuneHedgehog>,
    #[serde(rename = "distanceMax")]
    pub distance_max: Option<PortoCamel>,
    #[serde(rename = "distanceMin")]
    pub distance_min: Option<PortoCamel>,
    pub theta: Option<PortoCamel>,
    pub distance: Option<PointRadius>,
    pub id: Option<PurpleHammerfestPonies>,
    pub links: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrengthSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub expr: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortSignalRef {
    pub field: Option<SortField>,
    pub order: Option<SignalRefOrder>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub field: Option<String>,
    pub expr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VegaEncode {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutClass {
    pub align: Option<AlignUnion>,
    pub bounds: Option<BoundsUnion>,
    pub center: Option<Center>,
    pub columns: Option<PortoCamel>,
    #[serde(rename = "footerBand")]
    pub footer_band: Option<FooterBand>,
    #[serde(rename = "headerBand")]
    pub header_band: Option<HeaderBand>,
    pub offset: Option<LayoutOffset>,
    pub padding: Option<LayoutPadding>,
    #[serde(rename = "titleBand")]
    pub title_band: Option<TitleBand>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlignSignalRef {
    pub signal: Option<String>,
    pub column: Option<GridAlignUnion>,
    pub row: Option<GridAlignUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CenterSignalRef {
    pub signal: Option<String>,
    pub column: Option<PurpleTartuGecko>,
    pub row: Option<PurpleTartuGecko>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FooterBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<PortoCamel>,
    pub row: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<PortoCamel>,
    pub row: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetSignalRef {
    pub signal: Option<String>,
    #[serde(rename = "columnFooter")]
    pub column_footer: Option<PortoCamel>,
    #[serde(rename = "columnHeader")]
    pub column_header: Option<PortoCamel>,
    #[serde(rename = "columnTitle")]
    pub column_title: Option<PortoCamel>,
    #[serde(rename = "rowFooter")]
    pub row_footer: Option<PortoCamel>,
    #[serde(rename = "rowHeader")]
    pub row_header: Option<PortoCamel>,
    #[serde(rename = "rowTitle")]
    pub row_title: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaddingSignalRef {
    pub signal: Option<String>,
    pub column: Option<PortoCamel>,
    pub row: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleBandSignalRef {
    pub signal: Option<String>,
    pub column: Option<PortoCamel>,
    pub row: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegendElement {
    #[serde(rename = "clipHeight")]
    pub clip_height: Option<PortoCamel>,
    #[serde(rename = "columnPadding")]
    pub column_padding: Option<PortoCamel>,
    pub columns: Option<PortoCamel>,
    #[serde(rename = "cornerRadius")]
    pub corner_radius: Box<Option<Box<ChiangMaiGoose>>>,
    pub direction: Option<Direction>,
    pub encode: Option<LegendEncode>,
    pub fill: Option<String>,
    #[serde(rename = "fillColor")]
    pub fill_color: Option<ColorRef>,
    pub format: Option<PurpleFormat>,
    #[serde(rename = "gradientLength")]
    pub gradient_length: Option<PortoCamel>,
    #[serde(rename = "gradientOpacity")]
    pub gradient_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "gradientStrokeColor")]
    pub gradient_stroke_color: Option<ColorRef>,
    #[serde(rename = "gradientStrokeWidth")]
    pub gradient_stroke_width: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "gradientThickness")]
    pub gradient_thickness: Option<PortoCamel>,
    #[serde(rename = "gridAlign")]
    pub grid_align: Option<GridAlignUnion>,
    #[serde(rename = "labelAlign")]
    pub label_align: Option<FluffyAlign>,
    #[serde(rename = "labelBaseline")]
    pub label_baseline: Option<FluffyBaseline>,
    #[serde(rename = "labelColor")]
    pub label_color: Option<ColorRef>,
    #[serde(rename = "labelFont")]
    pub label_font: Option<LabelFont>,
    #[serde(rename = "labelFontSize")]
    pub label_font_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelFontWeight")]
    pub label_font_weight: Option<FluffyFontWeight>,
    #[serde(rename = "labelLimit")]
    pub label_limit: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelOffset")]
    pub label_offset: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelOpacity")]
    pub label_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "labelOverlap")]
    pub label_overlap: Option<LabelOverlap>,
    pub offset: Box<Option<Box<ChiangMaiGoose>>>,
    pub opacity: Option<String>,
    pub orient: Option<LegendOrient>,
    pub padding: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "rowPadding")]
    pub row_padding: Option<PortoCamel>,
    pub shape: Option<String>,
    pub size: Option<String>,
    pub stroke: Option<String>,
    #[serde(rename = "strokeColor")]
    pub stroke_color: Option<ColorRef>,
    #[serde(rename = "strokeDash")]
    pub stroke_dash: Option<String>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "symbolFillColor")]
    pub symbol_fill_color: Option<ColorRef>,
    #[serde(rename = "symbolOffset")]
    pub symbol_offset: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "symbolOpacity")]
    pub symbol_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "symbolSize")]
    pub symbol_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "symbolStrokeColor")]
    pub symbol_stroke_color: Option<ColorRef>,
    #[serde(rename = "symbolStrokeWidth")]
    pub symbol_stroke_width: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "symbolType")]
    pub symbol_type: Option<LabelFont>,
    #[serde(rename = "tickCount")]
    pub tick_count: Option<TickCount>,
    pub title: Option<PurpleFormat>,
    #[serde(rename = "titleAlign")]
    pub title_align: Option<FluffyAlign>,
    #[serde(rename = "titleBaseline")]
    pub title_baseline: Option<FluffyBaseline>,
    #[serde(rename = "titleColor")]
    pub title_color: Option<ColorRef>,
    #[serde(rename = "titleFont")]
    pub title_font: Option<LabelFont>,
    #[serde(rename = "titleFontSize")]
    pub title_font_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleFontWeight")]
    pub title_font_weight: Option<FluffyFontWeight>,
    #[serde(rename = "titleLimit")]
    pub title_limit: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titleOpacity")]
    pub title_opacity: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "titlePadding")]
    pub title_padding: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "type")]
    pub v4_type: Option<LegendType>,
    pub values: Option<Values>,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegendEncode {
    pub entries: Option<Axis>,
    pub gradient: Option<Axis>,
    pub labels: Option<Axis>,
    pub legend: Option<Axis>,
    pub symbols: Option<Axis>,
    pub title: Option<Axis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mark {
    #[serde(rename = "type")]
    pub mark_type: String,
    pub clip: Option<Clip>,
    pub encode: Option<VegaEncode>,
    pub interactive: Option<PurpleTartuGecko>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub on: Option<Vec<MarkOn>>,
    pub role: Option<String>,
    pub sort: Option<MarkSort>,
    pub style: Option<Style>,
    pub transform: Option<Vec<MarkTransform>>,
    pub axes: Option<Vec<AxeElement>>,
    pub data: Option<Vec<InputDataSetDefinition>>,
    pub layout: Option<LayoutClass>,
    pub legends: Option<Vec<LegendElement>>,
    pub marks: Option<Vec<Mark>>,
    pub projections: Option<Vec<ProjectionElement>>,
    pub scales: Option<Vec<ScaleMapping>>,
    pub signals: Option<Vec<SignalElement>>,
    pub title: Option<Title>,
    pub from: Option<From>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipSignalRef {
    pub signal: Option<String>,
    pub path: Option<PurpleFormat>,
    pub sphere: Option<PurpleFormat>,
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
pub struct MarkOn {
    pub modify: Option<String>,
    pub trigger: String,
    pub values: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectionElement {
    pub center: Option<CenterUnion>,
    #[serde(rename = "clipAngle")]
    pub clip_angle: Option<PortoCamel>,
    #[serde(rename = "clipExtent")]
    pub clip_extent: Option<Extent>,
    pub extent: Option<Extent>,
    pub fit: Option<Fit>,
    pub name: String,
    pub parallels: Option<CenterUnion>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<PortoCamel>,
    pub precision: Option<PortoCamel>,
    pub rotate: Option<CenterUnion>,
    pub scale: Option<PortoCamel>,
    pub size: Option<CenterUnion>,
    pub translate: Option<CenterUnion>,
    #[serde(rename = "type")]
    pub v4_type: Option<PurpleFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleMapping {
    pub domain: Option<ScaleMappingDomain>,
    #[serde(rename = "domainMax")]
    pub domain_max: Option<PortoCamel>,
    #[serde(rename = "domainMid")]
    pub domain_mid: Option<PortoCamel>,
    #[serde(rename = "domainMin")]
    pub domain_min: Option<PortoCamel>,
    #[serde(rename = "domainRaw")]
    pub domain_raw: Option<Values>,
    pub name: String,
    pub reverse: Option<PurpleTartuGecko>,
    pub round: Option<PurpleTartuGecko>,
    #[serde(rename = "type")]
    pub scale_mapping_type: Option<ExpressionString>,
    #[serde(rename = "domainImplicit")]
    pub domain_implicit: Option<PurpleTartuGecko>,
    pub range: Option<RangeUnion>,
    pub align: Option<PortoCamel>,
    pub padding: Option<PortoCamel>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<PortoCamel>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<PortoCamel>,
    pub clamp: Option<PurpleTartuGecko>,
    pub nice: Option<Nice>,
    pub zero: Option<PurpleTartuGecko>,
    pub interpolate: Option<Interpolate>,
    pub base: Option<PortoCamel>,
    pub exponent: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainSignalRef {
    pub data: Option<String>,
    pub field: Option<PurpleFormat>,
    pub sort: Option<SortUnion>,
    pub fields: Option<Vec<FluffyFormat>>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSignalRef {
    pub signal: Option<String>,
    pub data: Option<String>,
    pub field: Option<PurpleFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortSort {
    pub field: Option<PurpleFormat>,
    pub op: Option<PurpleFormat>,
    pub order: Option<PurpleOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterpolateSignalRef {
    pub signal: Option<String>,
    pub gamma: Option<PortoCamel>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<PurpleFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NiceSignalRef {
    pub interval: Option<IntervalUnion>,
    pub step: Option<PortoCamel>,
    pub signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeSignalRef {
    pub signal: Option<String>,
    pub count: Option<PortoCamel>,
    pub extent: Option<CenterUnion>,
    pub scheme: Option<PurpleFormat>,
    pub data: Option<String>,
    pub field: Option<PurpleFormat>,
    pub sort: Option<SortUnion>,
    pub fields: Option<Vec<FluffyFormat>>,
    pub step: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalElement {
    pub description: Option<String>,
    pub name: String,
    pub on: Option<Vec<SignalOn>>,
    pub push: Option<Push>,
    pub bind: Option<Bind>,
    pub react: Option<bool>,
    pub update: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bind {
    pub debounce: Option<f64>,
    pub element: Option<String>,
    pub input: Option<serde_json::Value>,
    pub name: Option<String>,
    pub options: Option<Vec<Option<serde_json::Value>>>,
    pub max: Option<f64>,
    pub min: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalOn {
    pub events: Events,
    pub force: Option<bool>,
    pub encode: Option<String>,
    pub update: Option<Update>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventElement {
    pub signal: Option<String>,
    pub scale: Option<String>,
    pub between: Option<Vec<InputEventStreamDefinition>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<String>,
    pub stream: Box<Option<InputEventStreamDefinition>>,
    pub merge: Option<Vec<InputEventStreamDefinition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputEventStreamDefinition {
    pub between: Option<Vec<InputEventStreamDefinition>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub input_event_stream_definition_type: Option<String>,
    pub stream: Box<Option<InputEventStreamDefinition>>,
    pub merge: Option<Vec<InputEventStreamDefinition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRef {
    pub signal: Option<String>,
    pub scale: Option<String>,
    pub between: Option<Vec<InputEventStreamDefinition>>,
    pub consume: Option<bool>,
    pub debounce: Option<f64>,
    pub filter: Option<Style>,
    pub markname: Option<String>,
    pub marktype: Option<String>,
    pub throttle: Option<f64>,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub signal_ref_type: Option<String>,
    pub stream: Box<Option<InputEventStreamDefinition>>,
    pub merge: Option<Vec<InputEventStreamDefinition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateExpressionRef {
    #[serde(rename = "as")]
    pub ref_as: Option<String>,
    pub expr: Option<String>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkSort {
    pub field: Option<SortField>,
    pub order: Option<SignalRefOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleClass {
    pub align: Option<FluffyAlign>,
    pub anchor: Option<AnchorUnion>,
    pub angle: Box<Option<Box<ChiangMaiGoose>>>,
    pub baseline: Option<FluffyBaseline>,
    pub color: Option<ColorRef>,
    pub encode: Option<TitleEncode>,
    pub font: Option<LabelFont>,
    #[serde(rename = "fontSize")]
    pub font_size: Box<Option<Box<ChiangMaiGoose>>>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<FluffyFontWeight>,
    pub frame: Option<TitleFrame>,
    pub interactive: Option<bool>,
    pub limit: Box<Option<Box<ChiangMaiGoose>>>,
    pub name: Option<String>,
    pub offset: Box<Option<Box<ChiangMaiGoose>>>,
    pub orient: Option<LegendOrient>,
    pub style: Option<Style>,
    pub text: TextElement,
    pub zindex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorElement {
    pub test: Option<String>,
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BraggadociousSignalRef {
    pub scale: Box<Option<Box<FieldRef>>>,
    pub signal: Option<String>,
    pub value: Option<serde_json::Value>,
    pub field: Box<Option<Box<FieldRef>>>,
    pub range: Option<Band>,
    pub band: Option<serde_json::Value>,
    pub offset: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleEncode {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkTransform {
    pub anchor: Option<PortoCamel>,
    #[serde(rename = "as")]
    pub v4_as: Option<As>,
    pub base: Option<PortoCamel>,
    pub divide: Option<CenterUnion>,
    pub extent: Option<Values>,
    pub field: Option<PurpleHammerfestPonies>,
    pub maxbins: Option<PortoCamel>,
    pub minstep: Option<PortoCamel>,
    pub name: Option<PurpleFormat>,
    pub nice: Option<PurpleTartuGecko>,
    pub signal: Option<String>,
    pub step: Option<PortoCamel>,
    pub steps: Option<CenterUnion>,
    #[serde(rename = "type")]
    pub v4_type: TransformType,
    pub sort: Option<FluffyTartuGecko>,
    pub expr: Option<String>,
    pub initonly: Option<PurpleTartuGecko>,
    pub fields: Option<FieldsUnion>,
    pub groupby: Option<GroupbyUnion>,
    pub key: Option<PurpleHammerfestPonies>,
    pub ops: Option<Ops>,
    #[serde(rename = "default")]
    pub v4_default: Option<serde_json::Value>,
    pub from: Option<String>,
    pub values: Option<GroupbyUnion>,
    pub size: Option<StepUnion>,
    pub frame: Option<ParamsUnion>,
    #[serde(rename = "ignorePeers")]
    pub ignore_peers: Option<PurpleTartuGecko>,
    pub params: Option<ParamsUnion>,
    pub orient: Option<OrientUnion>,
    pub require: Option<PurpleSignalRef>,
    pub shape: Option<ShapeUnion>,
    #[serde(rename = "sourceX")]
    pub source_x: Option<PurpleHammerfestPonies>,
    #[serde(rename = "sourceY")]
    pub source_y: Option<PurpleHammerfestPonies>,
    #[serde(rename = "targetX")]
    pub target_x: Option<PurpleHammerfestPonies>,
    #[serde(rename = "targetY")]
    pub target_y: Option<PurpleHammerfestPonies>,
    #[serde(rename = "endAngle")]
    pub end_angle: Option<PortoCamel>,
    #[serde(rename = "startAngle")]
    pub start_angle: Option<PortoCamel>,
    pub offset: Option<TransformOffset>,
    pub geojson: Option<PurpleHammerfestPonies>,
    #[serde(rename = "pointRadius")]
    pub point_radius: Option<PointRadius>,
    pub projection: Option<String>,
    pub alpha: Option<PortoCamel>,
    #[serde(rename = "alphaMin")]
    pub alpha_min: Option<PortoCamel>,
    #[serde(rename = "alphaTarget")]
    pub alpha_target: Option<PortoCamel>,
    pub forces: Option<Vec<ForceElement>>,
    pub iterations: Option<PortoCamel>,
    pub restart: Option<PurpleTartuGecko>,
    #[serde(rename = "static")]
    pub v4_static: Option<PurpleTartuGecko>,
    #[serde(rename = "velocityDecay")]
    pub velocity_decay: Option<PortoCamel>,
    pub padding: Option<FluffyPuneHedgehog>,
    pub radius: Option<PurpleHammerfestPonies>,
    pub round: Option<PurpleTartuGecko>,
    #[serde(rename = "parentKey")]
    pub parent_key: Option<PurpleHammerfestPonies>,
    pub method: Option<MethodUnion>,
    #[serde(rename = "nodeSize")]
    pub node_size: Option<CenterUnion>,
    pub separation: Option<PurpleTartuGecko>,
    #[serde(rename = "paddingBottom")]
    pub padding_bottom: Option<PortoCamel>,
    #[serde(rename = "paddingInner")]
    pub padding_inner: Option<PortoCamel>,
    #[serde(rename = "paddingLeft")]
    pub padding_left: Option<PortoCamel>,
    #[serde(rename = "paddingOuter")]
    pub padding_outer: Option<PortoCamel>,
    #[serde(rename = "paddingRight")]
    pub padding_right: Option<PortoCamel>,
    #[serde(rename = "paddingTop")]
    pub padding_top: Option<PortoCamel>,
    pub ratio: Option<PortoCamel>,
    pub x: Option<PurpleHammerfestPonies>,
    pub y: Option<PurpleHammerfestPonies>,
    pub font: Option<PurpleHammerfestPonies>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<PointRadius>,
    #[serde(rename = "fontSizeRange")]
    pub font_size_range: Option<CenterUnion>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<PurpleHammerfestPonies>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<PurpleHammerfestPonies>,
    pub rotate: Option<PointRadius>,
    pub spiral: Option<PurpleFormat>,
    pub text: Option<PurpleHammerfestPonies>,
    pub query: Option<Values>,
    pub filter: Option<serde_json::Value>,
    pub ignore: Option<PortoCamel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaddingClass {
    pub bottom: Option<f64>,
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Autosize {
    AutosizeClass(AutosizeClass),
    Enum(AutosizeEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChiangMaiGoose {
    BarcelonaHawaiianMonkSeal(BarcelonaHawaiianMonkSeal),
    BasseTerreElephantArray(Vec<BasseTerreElephant>),
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
pub enum FieldRef {
    FieldRefSignalRef(FieldRefSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorRef {
    ColorClass(ColorClass),
    FluffySignalRefArray(Vec<FluffySignalRef>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleV4 {
    BarcelonaHawaiianMonkSeal(BarcelonaHawaiianMonkSeal),
    BasseTerreElephantArray(Vec<BasseTerreElephant>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Style {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleFormat {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AxeGridDash {
    StickySignalRef(StickySignalRef),
    UnionArray(Vec<GridDashElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridDashElement {
    Double(f64),
    TentacledSignalRef(TentacledSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyAlign {
    Enum(PurpleAlign),
    IndecentSignalRef(IndecentSignalRef),
    IndigoSignalRefArray(Vec<IndigoSignalRef>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyBaseline {
    AmbitiousSignalRef(AmbitiousSignalRef),
    Enum(PurpleBaseline),
    HilariousSignalRefArray(Vec<HilariousSignalRef>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelBound {
    Bool(bool),
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortoCamel {
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelFont {
    CunningSignalRef(CunningSignalRef),
    FontElementArray(Vec<FontElement>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyFontWeight {
    Double(f64),
    Enum(PurpleFontWeight),
    FriskySignalRef(FriskySignalRef),
    MagentaSignalRefArray(Vec<MagentaSignalRef>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelOverlap {
    Bool(bool),
    Enum(LabelOverlapEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickCount {
    Double(f64),
    Enum(PurpleInterval),
    TickCountSignalRef(TickCountSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntervalUnion {
    Enum(PurpleInterval),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleTartuGecko {
    Bool(bool),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickRound {
    Bool(bool),
    MischievousSignalRef(MischievousSignalRef),
    TickRoundElementArray(Vec<TickRoundElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Values {
    AnythingArray(Vec<Option<serde_json::Value>>),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParseUnion {
    Enum(ParseEnum),
    StringMap(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Remove {
    Bool(bool),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaseUnion {
    Enum(CaseEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleHammerfestPonies {
    PurpleRef(PurpleRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterUnion {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<PurplePuneHedgehog>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurplePuneHedgehog {
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldsUnion {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<Option<FluffyHammerfestPonies>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyHammerfestPonies {
    FieldSignalRefClass(FieldSignalRefClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointRadius {
    Double(f64),
    FluffyRef(FluffyRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyPuneHedgehog {
    Double(f64),
    StrengthSignalRef(StrengthSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledPuneHedgehog {
    Double(f64),
    String(String),
    XSignalRef(XSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamsUnion {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<Option<PortoCamel>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupbyUnion {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<TentacledHammerfestPonies>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledHammerfestPonies {
    PurpleRef(PurpleRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformOffset {
    Enum(OffsetEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformOp {
    Enum(PurpleOp),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ops {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<OpElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpElement {
    Enum(FluffyOp),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrientUnion {
    Enum(Direction),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShapeUnion {
    Enum(ShapeEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepUnion {
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<PurplePuneHedgehog>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyTartuGecko {
    Bool(bool),
    SortSignalRef(SortSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortField {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
    UnionArray(Vec<TextElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextElement {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SignalRefOrder {
    Enum(OrderEnum),
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<OrderElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderElement {
    Enum(OrderEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum As {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
    UnionArray(Vec<Option<PurpleFormat>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleFields {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<StickyHammerfestPonies>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyHammerfestPonies {
    Double(f64),
    String(String),
    ValueSignalRef(ValueSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlignUnion {
    AlignSignalRef(AlignSignalRef),
    Enum(AlignEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridAlignUnion {
    Enum(AlignEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoundsUnion {
    Enum(BoundsEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Center {
    Bool(bool),
    CenterSignalRef(CenterSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FooterBand {
    Double(f64),
    FooterBandSignalRef(FooterBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeaderBand {
    Double(f64),
    HeaderBandSignalRef(HeaderBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutOffset {
    Double(f64),
    OffsetSignalRef(OffsetSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutPadding {
    Double(f64),
    PaddingSignalRef(PaddingSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleBand {
    Double(f64),
    TitleBandSignalRef(TitleBandSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Clip {
    Bool(bool),
    ClipSignalRef(ClipSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Extent {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<ClipExtentElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClipExtentElement {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<PurplePuneHedgehog>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Fit {
    AnythingArray(Vec<Option<serde_json::Value>>),
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleMappingDomain {
    DomainSignalRef(DomainSignalRef),
    UnionArray(Vec<Option<DomainElement>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainElement {
    Bool(bool),
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyFormat {
    FieldSignalRef(FieldSignalRef),
    String(String),
    UnionArray(Vec<Field>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    Bool(bool),
    Double(f64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortUnion {
    Bool(bool),
    SortSort(SortSort),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleOrder {
    Enum(OrderEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Interpolate {
    InterpolateSignalRef(InterpolateSignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nice {
    Bool(bool),
    Double(f64),
    Enum(PurpleInterval),
    NiceSignalRef(NiceSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeUnion {
    Enum(RangeEnum),
    RangeSignalRef(RangeSignalRef),
    UnionArray(Vec<Option<DomainElement>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    EventElementArray(Vec<EventElement>),
    SignalRef(SignalRef),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Update {
    String(String),
    UpdateExpressionRef(UpdateExpressionRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Title {
    String(String),
    TitleClass(TitleClass),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnchorUnion {
    AnchorElementArray(Vec<AnchorElement>),
    BraggadociousSignalRef(BraggadociousSignalRef),
    Enum(AnchorEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleFrame {
    CunningSignalRef(CunningSignalRef),
    Enum(FrameEnum),
    FontElementArray(Vec<FontElement>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MethodUnion {
    Enum(MethodEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VegaPadding {
    Double(f64),
    PaddingClass(PaddingClass),
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
pub enum PurpleAlign {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleBaseline {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleFontWeight {
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
pub enum AxeOrient {
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
pub enum PurpleInterval {
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
pub enum ParseEnum {
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FormatType {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "dsv")]
    Dsv,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "topojson")]
    Topojson,
    #[serde(rename = "tsv")]
    Tsv,
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
    #[serde(rename = "nth_value")]
    NthValue,
    #[serde(rename = "ntile")]
    Ntile,
    #[serde(rename = "__count__")]
    OpCount,
    #[serde(rename = "percent_rank")]
    PercentRank,
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
pub enum Direction {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "radial")]
    Radial,
    #[serde(rename = "vertical")]
    Vertical,
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
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "impute")]
    Impute,
    #[serde(rename = "joinaggregate")]
    Joinaggregate,
    #[serde(rename = "linkpath")]
    Linkpath,
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
pub enum AlignEnum {
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
pub enum LegendOrient {
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
pub enum LegendType {
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "symbol")]
    Symbol,
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
pub enum ExpressionString {
    #[serde(rename = "band")]
    Band,
    #[serde(rename = "bin-linear")]
    BinLinear,
    #[serde(rename = "bin-ordinal")]
    BinOrdinal,
    #[serde(rename = "identity")]
    Identity,
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
pub enum AnchorEnum {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "start")]
    Start,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FrameEnum {
    #[serde(rename = "bounds")]
    Bounds,
    #[serde(rename = "group")]
    Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MethodEnum {
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "dice")]
    Dice,
    #[serde(rename = "resquarify")]
    Resquarify,
    #[serde(rename = "slice")]
    Slice,
    #[serde(rename = "slicedice")]
    Slicedice,
    #[serde(rename = "squarify")]
    Squarify,
    #[serde(rename = "tidy")]
    Tidy,
}
