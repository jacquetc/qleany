from pathlib import Path

import stringcase

from qleany_.common.entities.entity import Entity
from qleany_.common.entities.feature import Feature
from qleany_.common.entities.field import Field
from qleany_.common.entities.root import Root
from qleany_.manifest_handling_feature.dtos import LoadManifestDto
from qleany_.manifest_handling_feature.i_importer import IImporter
from qleany_.manifest_handling_feature.i_manifest_handling_uow import (
    IManifestHandlingUow,
)


class LoadUc:
    def __init__(self, unit_of_work: IManifestHandlingUow, importer: IImporter):
        self._unit_of_work = unit_of_work
        self._importer = importer

    def execute(self, dto: LoadManifestDto):
        self.validate(dto)
        print(f"Loading manifest from {dto.file_path}")

        self._importer.load_file(Path(dto.file_path))

        with self._unit_of_work as uow:
            # clear repositories
            uow.root_repository.clear()
            uow.entity_repository.clear()
            uow.feature_repository.clear()
            uow.field_repository.clear()

            # create root
            root = uow.root_repository.create([Root(id_=1)])[0]

            manifest = self._importer.get_json()

            # create entities
            json_entities = manifest["entities"]["list"]
            entities_to_create = []
            for json_entity in json_entities:
                entities_to_create.append(
                    Entity(
                        id_=0,
                        name=json_entity["name"],
                        only_for_heritage=json_entity.get("only_for_heritage", False),
                    )
                )

            new_entities = uow.entity_repository.create(
                entities=entities_to_create, owner_id=root.id_, position=0
            )

            for json_entity in json_entities:
                # find the equivalent in new_entities
                entity = next(
                    (e for e in new_entities if e.name == json_entity["name"]), None
                )
                if entity is None:
                    raise ValueError(f"Entity {json_entity['name']} not found")

                # add fields
                fields_to_create = []
                for json_field in json_entity["fields"]:
                    fields_to_create.append(
                        Field(
                            id_=0,
                            name=json_field["name"],
                            type_=json_field["type"],
                            entity=None,  # to be set later
                            is_nullable=json_field.get("is_nullable", False),
                            is_primary_key=True
                            if json_field.get(json_field["name"], "") == "id"
                            else False,
                            is_list=json_field.get("is_list", False),
                            is_single=False,  # to be set later
                            strong=json_field.get("strong", False),
                            ordered=json_field.get("ordered", False),
                            list_model=json_field.get("list_model", False),
                            list_model_displayed_field=json_field.get(
                                "list_model_displayed_field", ""
                            ),
                        )
                    )
                # create fields and add them to the entity
                new_fields = uow.field_repository.create(
                    entities=fields_to_create, owner_id=entity.id_, position=0
                )

                # set entity for field.entity
                for json_field, new_field in zip(json_entity["fields"], new_fields):
                    related_entity = json_field.get("entity", None)
                    if related_entity is None:
                        continue
                    related_entity = next(
                        (e for e in new_entities if e.name == related_entity), None
                    )
                    if related_entity is None:
                        raise ValueError(f"Entity {related_entity} not found")
                    new_field.entity = related_entity.id_

                # set is_single
                for new_field in new_fields:
                    if new_field.entity is not None and not new_field.is_list:
                        new_field.is_single = True

                uow.field_repository.update(entities=new_fields)

            # add features
            features_to_create = []
            for json_feature, feature_data in manifest["features"].items():
                features_to_create.append(
                    Feature(
                        id_=0,
                        name=stringcase.pascalcase(json_feature),
                        description=feature_data.get("description", ""),
                        # use_cases=[] # to be set later
                    )
                )
            # create features and add them to the root
            new_features = uow.feature_repository.create(
                entities=features_to_create, owner_id=root.id_, position=0
            )

    def validate(self, dto: LoadManifestDto):
        pass
