def is_unique_foreign_entity(field_type: str, entities_by_name: dict) -> bool:
    for entity_name, _ in entities_by_name.items():
        if entity_name == field_type:
            return True

    return False


def is_list_foreign_entity(field_type: str, entities_by_name: dict) -> bool:
    if "<" not in field_type:
        return False

    type = field_type.split("<")[1].split(">")[0].strip()

    for entity_name, _ in entities_by_name.items():
        if entity_name == type:
            return True

    return False


def does_entity_have_relation_fields(entity_name: str, entities_by_name: dict) -> bool:
    entity = entities_by_name.get(entity_name, None)
    if entity is None:
        return False

    fields = entity["fields"]
    for field in fields:
        field_type = field["type"]
        if is_unique_foreign_entity(
            field_type, entities_by_name
        ) or is_list_foreign_entity(field_type, entities_by_name):
            return True

    return False


def get_entity_from_foreign_field_type(field_type: str, entities_by_name: dict) -> str:
    if "<" not in field_type:
        return field_type

    type = field_type.split("<")[1].split(">")[0].strip()

    for entity_name in entities_by_name:
        if entity_name == type:
            return entity_name

    return ""


def determine_owner(entity_name: str, entities_by_name: dict) -> dict:
    owner_dict = {}
    for possible_owner_name, entity in entities_by_name.items():
        for field in entity["fields"]:
            if field["type"] == entity_name or field["type"] == f"QList<{entity_name}>":
                if field.get("strong", False):
                    owner_dict["name"] = possible_owner_name
                    owner_dict["field"] = field["name"]
                    owner_dict["ordered"] = field.get("ordered", False)
                    owner_dict["is_list"] = field["type"] == f"QList<{entity_name}>"
                    return owner_dict

    return owner_dict