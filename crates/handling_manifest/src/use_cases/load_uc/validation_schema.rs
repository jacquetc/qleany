use serde_json::json;

pub fn json_validation_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "schema": {
                "type": "object",
                "properties": {
                    "version": {
                        "type": "integer",
                        "minimum": 2,
                        "maximum": 2
                    }
                },
                "required": ["version"]
            },
            "global": {
                "type": "object",
                "properties": {
                    "language": {
                        "type": "string",
                        "enum": ["python", "rust"]
                    },
                    "application_name": {
                        "type": "string"
                    },
                    "organisation": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string"
                            },
                            "domain": {
                                "type": "string"
                            }
                        },
                        "required": ["name", "domain"]
                    },
                    "prefix_path": {
                        "type": "string"
                    }
                },
                "required": ["language", "application_name", "organisation", "prefix_path"]
            },
            "entities": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        },
                        "parent": {
                            "type": "string"
                        },
                        "only_for_heritage": {
                            "type": "boolean"
                        },
                        "fields": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "type": {
                                        "type": "string"
                                    },
                                    "entity": {
                                        "type": "string"
                                    },
                                    "is_list": {
                                        "type": "boolean"
                                    },
                                    "ordered": {
                                        "type": "boolean"
                                    },
                                    "strong": {
                                        "type": "boolean"
                                    },
                                    "list_model": {
                                        "type": "boolean"
                                    },
                                    "list_model_displayed_field": {
                                        "type": "string"
                                    },
                                    "is_nullable": {
                                        "type": "boolean"
                                    },
                                    "is_primary_key": {
                                        "type": "boolean"
                                    },
                                    "enum_values": {
                                        "type": "array",
                                        "items": {
                                            "type": "string"
                                        }
                                    },
                                },
                                "required": ["name", "type"]
                            }
                        }
                    },
                    "required": ["name", "fields"]
                }
            },
            "features": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        },
                        "use_cases": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "validator": {
                                        "type": "boolean"
                                    },
                                    "entities": {
                                        "type": "array",
                                        "items": {
                                            "type": "string"
                                        }
                                    },
                                    "undoable": {
                                        "type": "boolean"
                                    },
                                    "read_only": {
                                        "type": "boolean"
                                    },
                                    "long_operation": {
                                        "type": "boolean"
                                    },
                                    "dto_in": {
                                        "type": "object",
                                        "properties": {
                                            "name": {
                                                "type": "string"
                                            },
                                            "fields": {
                                                "type": "array",
                                                "items": {
                                                    "type": "object",
                                                    "properties": {
                                                        "name": {
                                                            "type": "string"
                                                        },
                                                        "type": {
                                                            "type": "string"
                                                        },
                                                        "enum_values": {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "string"
                                                            }
                                                        },
                                                    },
                                                    "required": ["name", "type"]
                                                }
                                            }
                                        },
                                        "required": ["name", "fields"]
                                    },
                                    "dto_out": {
                                        "type": "object",
                                        "properties": {
                                            "name": {
                                                "type": "string"
                                            },
                                            "fields": {
                                                "type": "array",
                                                "items": {
                                                    "type": "object",
                                                    "properties": {
                                                        "name": {
                                                            "type": "string"
                                                        },
                                                        "type": {
                                                            "type": "string"
                                                        },
                                                        "is_list": {
                                                            "type": "boolean"
                                                        },
                                                        "enum_values": {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "string"
                                                            }
                                                        },
                                                    },
                                                    "required": ["name", "type"]
                                                }
                                            }
                                        },
                                        "required": ["name", "fields"]
                                    }
                                },
                                "required": ["name", "validator", "undoable"]
                            }
                        }
                    },
                    "required": ["name", "use_cases"]
                }
            },
            "ui": {
                "type": "object",
                "properties": {
                    "cli": {
                        "type": "boolean"
                    }
                },
                "required": ["cli"]
            }
        },
        "required": ["schema", "global", "entities", "features", "ui"]
    })
}
