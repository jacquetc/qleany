import stringcase
import copy


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


def does_entity_have_relation_fields(entity_name: str, entities_by_name: dict, keep_hidden: bool = True) -> bool:
    entity = entities_by_name.get(entity_name, None)
    if entity is None:
        return False

    fields = entity["fields"]
    for field in fields:
        field_type = field["type"]
        if keep_hidden is False and field.get("hidden", False):
            continue
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
                    owner_dict["hidden"] = field.get("hidden", False)
                    owner_dict["is_list"] = field["type"] == f"QList<{entity_name}>"
                    return owner_dict

    return owner_dict


def get_fields_with_foreign_entities(
    fields: list, entities_by_name: dict, target_entity: str = ""
) -> list:
    if fields is None:
        return []

    # make a deep copy of the fields
    fields = copy.deepcopy(fields)

    # get recursive fields from parent

    parent_fields = []
    entity_parent = entities_by_name.get(target_entity, {}).get("parent", "")

    while entity_parent:
        parent_fields = (
            get_entity_fields(entity_parent, entities_by_name) + parent_fields
        )
        entity_parent = entities_by_name.get(entity_parent, {}).get("parent", "")

    fields = parent_fields + fields

    # add fields with foreign entities
    for field in fields:
        field["pascal_name"] = stringcase.pascalcase(field["name"])
        field["hidden"] = field.get("hidden", False)

        if is_unique_foreign_entity(
            field["type"], entities_by_name
        ) or is_list_foreign_entity(field["type"], entities_by_name):
            field["is_foreign"] = True

            # get foreign entity name
            foreign_entity_name = get_entity_from_foreign_field_type(
                field["type"], entities_by_name
            )
            field["foreign_dto_type"] = f"{foreign_entity_name}DTO"
            field["entity_type"] = field["type"]
            field["type"] = (
                f"{foreign_entity_name}DTO"
                if field["type"].count(">") == 0
                else f"QList<{foreign_entity_name}DTO>"
            )

        else:
            field["is_foreign"] = False

    return fields


def get_fields_without_foreign_entities(
    fields: list, entities_by_name: dict, target_entity: str = ""
) -> list:
    # make a deep copy of the fields
    fields = copy.deepcopy(fields)

    # get recursive fields from parent

    parent_fields = []
    entity_parent = entities_by_name.get(target_entity, {}).get("parent", "")

    while entity_parent:
        parent_fields = (
            get_entity_fields(entity_parent, entities_by_name) + parent_fields
        )
        entity_parent = entities_by_name.get(entity_parent, {}).get("parent", "")

    fields = parent_fields + fields

    # add fields without foreign entities
    fields_without_foreign = []
    for field in fields:
        field["pascal_name"] = stringcase.pascalcase(field["name"])
        field["snake_name"] = stringcase.snakecase(field["name"])
        field["spinal_name"] = stringcase.spinalcase(field["name"])
        field["camel_name"] = stringcase.camelcase(field["name"])
        field["hidden"] = field.get("hidden", False)

        if is_unique_foreign_entity(
            field["type"], entities_by_name
        ) or is_list_foreign_entity(field["type"], entities_by_name):
            continue

        else:
            field["default_value_for_qml"] = get_default_value_for_qml(field["type"])
            field["is_foreign"] = False
            fields_without_foreign.append(field)

    return fields_without_foreign


def get_default_value_for_qml(type: str):
    if type == "int":
        return "0"
    elif type == "double":
        return "0.0"
    elif type == "bool":
        return "false"
    elif type == "QString":
        return '"example"'
    elif type == "QDate":
        # format "YYYY-MM-DD"
        return '"2020-01-01"'
    elif type == "QDateTime":
        # format "YYYY-MM-DDThh:mm:ss"
        return '"2020-01-01T00:00:00"'
    elif type == "QUrl":
        return '""'
    elif type == "QUuid":
        return '""'
    elif type == "QByteArray":
        return '""'
    elif type == "QVariant":
        return '""'
    elif type == "QList<int>":
        return "[]"
    elif type == "QList<double>":
        return "[]"
    elif type == "QList<bool>":
        return "[]"
    elif type == "QList<QString>":
        return "[]"
    elif type == "QList<QDate>":
        return "[]"
    elif type == "QList<QTime>":
        return "[]"
    elif type == "QList<QDateTime>":
        return "[]"
    elif type == "QList<QUrl>":
        return "[]"
    elif type == "QList<QByteArray>":
        return "[]"
    elif type == "QList<QVariant>":
        return "[]"
    else:
        return "null"


def get_only_fields_with_foreign_entities(
    fields: list, entities_by_name: dict, target_entity: str = ""
) -> list:
    # make a deep copy of the fields
    fields = copy.deepcopy(fields)

    # get recursive fields from parent

    parent_fields = []
    entity_parent = entities_by_name.get(target_entity, {}).get("parent", "")

    while entity_parent:
        parent_fields = (
            get_entity_fields(entity_parent, entities_by_name) + parent_fields
        )
        entity_parent = entities_by_name.get(entity_parent, {}).get("parent", "")

    fields = parent_fields + fields

    # add fields without foreign entities
    fields_with_foreign = []
    for field in fields:
        field["pascal_name"] = stringcase.pascalcase(field["name"])

        if is_unique_foreign_entity(
            field["type"], entities_by_name
        ) or is_list_foreign_entity(field["type"], entities_by_name):
            field["is_foreign"] = True

            # get foreign entity name
            foreign_entity_name = get_entity_from_foreign_field_type(
                field["type"], entities_by_name
            )
            field["foreign_dto_type"] = f"{foreign_entity_name}DTO"
            field["entity_type"] = field["type"]
            field["type"] = (
                f"{foreign_entity_name}DTO"
                if field["type"].count(">") == 0
                else f"QList<{foreign_entity_name}DTO>"
            )
            fields_with_foreign.append(field)

        else:
            continue

    return fields_with_foreign


def get_entity_fields(entity_name: str, entities_by_name: dict) -> list:
    if entity_name == "EntityBase":
        return [{"name": "id", "type": "int", "pascal_name": "Id", "is_foreign": False}]

    entity_data = entities_by_name[entity_name]
    fields = entity_data["fields"]
    return fields
