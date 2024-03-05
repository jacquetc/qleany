from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format_runner as clang_format_runner
from pathlib import Path

def generate_blank_qleany_yaml(folder_path: str):
    qleany_yaml = """
schema:
  version: 1
  
global:
  application_name: example
  application_cpp_domain_name: example
  organisation:
    name: example
    domain: example.com

entities:
  folder_path: src/entities/
  list:
    - name: Entity
      parent: EntityBase
      only_for_heritage: true
      fields:
        - type: QUuid
          name: uuid
        - type: QDateTime
          name: creationDate
        - type: QDateTime
          name: updateDate
    - name: FirstEntity
      parent: Entity
      fields:
        - type: QString
          name: content
        - type: SecondEntity
          name: secondEntity
          strong: true
        - type: QList<ThirdEntity>
          name: bucketOfThirdEntities
          ordered: true
          strong: true
    - name: SecondEntity
      parent: Entity
      fields:
        - type: QString
          name: name
    - name: ThirdEntity
      parent: Entity
      fields:
        - type: QString
          name: name
        - type: int
          name: age

repositories:
  list:
      - entity_name: FirstEntity
        lazy_loaders: true
      - entity_name: SecondEntity
        lazy_loaders: true
      - entity_name: ThirdEntity
        lazy_loaders: true
  repository_folder_path: src/persistence/repository/
  base_folder_path: src/persistence/ 

interactor:
  folder_path: src/interactor/
  create_undo_redo_interactor: false

application:
  common_cmake_folder_path: src/application
  features:
    - name: FirstEntity
      DTO:
        dto_identical_to_entity:
          enabled: true
          entity_mappable_with: FirstEntity
      CRUD:
        enabled: true       
        entity_mappable_with: FirstEntity
        get:
          enabled: true
        get_all:
          enabled: true
        get_with_details:
          enabled: true
        create: 
          enabled: true
        remove: 
          enabled: true
        update:
          enabled: true
        insert_relation: 
          enabled: true
    - name: SecondEntity
      DTO:
        dto_identical_to_entity:
          enabled: true
          entity_mappable_with: SecondEntity
      CRUD:
        enabled: true       
        entity_mappable_with: SecondEntity
        get:
          enabled: true
        get_all:
          enabled: true
        get_with_details:
          enabled: false
        create: 
          enabled: true
        remove: 
          enabled: true
        update: 
          enabled: true
        change_active_status: 
          enabled: true
    - name: ThirdEntity
      DTO:
        dto_identical_to_entity:
          enabled: true
          entity_mappable_with: ThirdEntity
      CRUD:
        enabled: true       
        entity_mappable_with: ThirdEntity
        get:
          enabled: true
        get_all:
          enabled: true
        get_with_details:
          enabled: false
        create: 
          enabled: true
        remove: 
          enabled: true
        update: 
          enabled: true
        change_active_status: 
          enabled: true
    - name: Custom
      DTO:
        dto_identical_to_entity:
          enabled: false
      CRUD:
        enabled: false
      commands:
        - name: WriteRandomThings
          entities:
            - FirstEntity
            - SecondEntity
            - ThirdEntity
          validator: 
            enabled: True
          undo: False
          dto:
            in:
              type_prefix: WriteRandomThings
              fields:
                - type: QString
                  name: randomCarName
            out:
              enabled: false
        - name: RunLongOperation
          validator: 
            enabled: False
          undo: False
          minimum_progress_time: 1000
          dto:
            in:
              enabled: false
            out:
              enabled: false
        - name: CloseSystem
          entities:
            - FirstEntity
            - SecondEntity
            - ThirdEntity
          validator: 
            enabled: False
          undo: False
          dto:
            in:
              enabled: false
            out:
              enabled: false
      queries:
        - name: GetCurrentTime
          validator:
            enabled: false
          undo: False
          dto:
            in:
              enabled: false
            out:
              type_prefix: GetCurrentTimeReply
              fields:
                - type: QDateTime
                  name: currentDateTime

DTOs:
  common_cmake_folder_path: src/contracts.dto

CQRS:
  common_cmake_folder_path: src/contracts.cqrs

contracts:
  inverted_app_domain: com.example
  folder_path: src/contracts

presenter:
  folder_path: src/presenters
  create_undo_and_redo_singles: false
  singles:
    - name: SingleFirstEntity
      entity: FirstEntity
    - name: auto
      entity: SecondEntity
    - name: auto
      entity: ThirdEntity
  list_models:
    - name: ThirdEntityListModelFromFirstEntityBucketOfThirdEntities
      entity: ThirdEntity
      displayed_field: name
      in_relation_of: FirstEntity
      relation_field_name: bucketOfThirdEntities
    - name: FirstEntityListModel
      entity: FirstEntity  
      displayed_field: content

front_ends:
  qt_widgets:
    folder_path: src/qt_widgets_application/
"""

    with open(os.path.join(folder_path, "qleany.yaml"), "w") as file:
        file.write(qleany_yaml)