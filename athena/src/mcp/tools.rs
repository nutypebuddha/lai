use serde_json::Value;

pub struct AthenaTool {
    name: &'static str,
    description: &'static str,
    parameters: Value,
}

impl AthenaTool {
    pub fn new(name: &'static str, description: &'static str, parameters: Value) -> Self {
        AthenaTool {
            name,
            description,
            parameters,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
    pub fn description(&self) -> &str {
        self.description
    }
    pub fn parameters(&self) -> &Value {
        &self.parameters
    }
}

pub struct ToolRegistry {
    tools: Vec<AthenaTool>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut tools = vec![
            Self::validate_tool(),
            Self::traverse_tool(),
            Self::compose_tool(),
            Self::formula_search_tool(),
            Self::formula_by_output_tool(),
            Self::wheel_tool(),
            Self::reason_tool(),
            Self::evaluate_tool(),
            Self::classify_tool(),
            Self::gyro_tool(),
            Self::entity_list_tool(),
            Self::entity_get_tool(),
            Self::entity_aspect_tool(),
            Self::entity_search_tool(),
            Self::entity_eval_tool(),
            Self::ephemeris_tool(),
            Self::savings_tool(),
        ];
        #[cfg(feature = "budget")]
        {
            tools.push(Self::budget_stats_tool());
            tools.push(Self::budget_reset_tool());
        }
        ToolRegistry { tools }
    }

    pub fn all(&self) -> &[AthenaTool] {
        &self.tools
    }

    fn validate_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_validate",
            "Validate a claim through Athena's gates (math, logic, formal). Returns structured validation results.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {"type": "string", "description": "Expression or claim to validate"},
                    "gate": {"type": "string", "enum": ["math", "logic", "formal", "all"], "default": "math"}
                },
                "required": ["text"]
            }),
        )
    }

    fn traverse_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_traverse",
            "Traverse the zodiac wheel graph starting from a domain. Discovers formulas and relationships along the path.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "domain": {"type": "string", "description": "Starting domain (surya, chandra, mangala, etc.)"},
                    "max_depth": {"type": "integer", "default": 5, "description": "Maximum traversal depth"}
                },
                "required": ["domain"]
            }),
        )
    }

    fn compose_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_compose",
            "Compose two or more formulas into a reasoning chain. Validates the cross-domain aspects between them.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "formulas": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Ordered list of formula IDs to compose"
                    }
                },
                "required": ["formulas"]
            }),
        )
    }

    fn formula_search_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_formula_search",
            "Search the formula database by keyword. Returns matching formula IDs, domains, and descriptions.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "keyword": {"type": "string", "description": "Search keyword"},
                    "limit": {"type": "integer", "default": 25, "description": "Max results returned"},
                    "detail": {"type": "string", "enum": ["compact", "full"], "default": "compact", "description": "compact trims flavor text and extra fields"}
                },
                "required": ["keyword"]
            }),
        )
    }

    fn wheel_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_wheel",
            "Display the Vedic graha wheel graph structure. Shows all 9 domains (grahas), their symbols, and aspect relationships.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "domain": {"type": "string", "description": "Optional domain to show details for"},
                    "detail": {"type": "string", "enum": ["compact", "full"], "default": "compact"}
                }
            }),
        )
    }

    fn reason_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_reason",
            "Find the shortest formula chain to derive a desired variable from available variables. Optionally execute the chain.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "have": {"type": "string", "description": "Comma-separated variable names you already have values for"},
                    "want": {"type": "string", "description": "The variable name you want to derive"},
                    "max_depth": {"type": "integer", "default": 5, "description": "Maximum search depth"},
                    "execute": {"type": "boolean", "default": false, "description": "Whether to execute the chain with provided args"},
                    "args": {"type": "object", "description": "Key-value pairs of arguments (used with execute=true)", "additionalProperties": {"type": "number"}}
                },
                "required": ["have", "want"]
            }),
        )
    }

    fn evaluate_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_evaluate",
            "Evaluate a single formula with given numeric arguments.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "formula": {"type": "string", "description": "Formula ID (e.g. newtons_second)"},
                    "args": {"type": "object", "description": "Argument key-value pairs", "additionalProperties": {"type": "number"}}
                },
                "required": ["formula"]
            }),
        )
    }

    fn entity_list_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_entity_list",
            "List all entities in the knowledge graph. Returns entity IDs, names, domains, and descriptions.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {"type": "integer", "default": 25, "description": "Max entities returned"},
                    "detail": {"type": "string", "enum": ["compact", "full"], "default": "compact", "description": "compact returns id/name/kind/tags only"}
                },
            }),
        )
    }

    fn entity_get_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_entity_get",
            "Get detailed information about an entity by its ID. Returns properties, tags, domain context, and Vedic classification (graha, guna, nakshatra, mantra, bija).",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "Entity ID (e.g. schizoaffective_disorder, lithium)"}
                },
                "required": ["id"]
            }),
        )
    }

    fn entity_aspect_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_entity_aspect",
            "Compute the aspect relationship between two entities. Uses the zodiac wheel to determine their connection (conjunction, sextile, trine, square, opposition).",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "from": {"type": "string", "description": "First entity ID"},
                    "to": {"type": "string", "description": "Second entity ID"}
                },
                "required": ["from", "to"]
            }),
        )
    }

    fn entity_search_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_entity_search",
            "Search entities by keyword in their ID, name, description, tags, or Vedic classification (graha, guna, nakshatra, tattva, rashi).",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "keyword": {"type": "string", "description": "Search keyword"},
                    "limit": {"type": "integer", "default": 25, "description": "Max entities returned"},
                    "detail": {"type": "string", "enum": ["compact", "full"], "default": "compact"}
                },
                "required": ["keyword"]
            }),
        )
    }

    fn entity_eval_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_entity_eval",
            "Evaluate a formula grounded in an entity. Entity properties and constants fill missing formula arguments.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "formula": {"type": "string", "description": "Formula ID to evaluate"},
                    "entity": {"type": "string", "description": "Entity ID to ground the formula in"},
                    "args": {"type": "object", "description": "Additional argument key-value pairs", "additionalProperties": {"type": "number"}}
                },
                "required": ["formula", "entity"]
            }),
        )
    }

    fn classify_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_classify",
            "Classify a text token across all 7 astrology axes (Western + Vedic). Compact returns the dominant picks; detail:\"full\" adds all weighted score maps.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {"type": "string", "description": "Text to classify"},
                    "detail": {"type": "string", "enum": ["compact", "full"], "default": "compact"}
                },
                "required": ["text"]
            }),
        )
    }

    fn ephemeris_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_ephemeris",
            "Deterministic graha (planet) positions for a date: tropical + sidereal (Lahiri) longitudes, rashi, nakshatra, pada. VSOP87/ELP-2000, same input always yields the same output.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "date": {"type": "string", "description": "Gregorian date YYYY-MM-DD (UT)"},
                    "time": {"type": "string", "description": "Time HH:MM (UT), default 00:00"},
                    "graha": {"type": "string", "description": "Optional single graha (surya, chandra, mangala, budha, brihaspati, shukra, shani, rahu, ketu)"}
                },
                "required": ["date"]
            }),
        )
    }

    fn savings_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_savings",
            "Report cumulative context-token savings from response compaction this session (calls, baseline vs emitted tokens, saved total and percent).",
            serde_json::json!({
                "type": "object",
                "properties": {},
            }),
        )
    }

    #[cfg(feature = "budget")]
    fn budget_stats_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_budget_stats",
            "Show token budget usage: prompt/completion/total used vs limits, and per-spend records.",
            serde_json::json!({
                "type": "object",
                "properties": {},
            }),
        )
    }

    #[cfg(feature = "budget")]
    fn budget_reset_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_budget_reset",
            "Reset the token budget exceeded flag so blocked calls can proceed again.",
            serde_json::json!({
                "type": "object",
                "properties": {},
            }),
        )
    }

    fn gyro_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_gyro",
            "Get the current gyroscopic wheel state: orientation, dominant sign, precession rate, mass distribution, and alignment weights.",
            serde_json::json!({
                "type": "object",
                "properties": {},
            }),
        )
    }

    fn formula_by_output_tool() -> AthenaTool {
        AthenaTool::new(
            "athena_formula_by_output",
            "Find formulas by their output variable name. Reverse lookup: what formulas produce 'kinetic_energy'? Returns formula IDs, domains, and descriptions.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "output": {"type": "string", "description": "Output variable name to search for (e.g. kinetic_energy, ke, sum)"}
                },
                "required": ["output"]
            }),
        )
    }
}
