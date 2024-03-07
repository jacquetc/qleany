import ruamel.yaml
import json
import jsonschema


def validate_manifest(yaml_file: str) -> str:

    try:
        # load yaml file
        with open(yaml_file, "r") as file:
            yaml = ruamel.yaml.YAML(typ="safe", pure=True)
            yaml_manifest = yaml.load(file)

        # dump yaml to json
        json_manifest = json.loads(json.dumps(yaml_manifest))

        with open("manifest_schema.json", "r") as file:
            schema = json.load(file)

        jsonschema.validate(json_manifest, schema)

    except Exception as e:
        print(f"Error: {e}")
        return str(e)

    return None
