from sqlalchemy import Boolean, Column, ForeignKey, Integer, String
from sqlalchemy.orm import declarative_base, relationship

Base = declarative_base()

class Dto(Base):
    __tablename__ = 'dto'
    id_ = Column(Integer, primary_key=True)
    name = Column(String)
    fields = relationship("DtoField", back_populates="dto")

class DtoField(Base):
    __tablename__ = 'dto_field'
    id_ = Column(Integer, primary_key=True)
    name = Column(String)
    type_ = Column(String)
    is_nullable = Column(Boolean)
    is_list = Column(Boolean)
    dto_id = Column(Integer, ForeignKey('dto.id_'))
    dto = relationship("Dto", back_populates="fields")

class Entity(Base):
    __tablename__ = 'entity'
    id_ = Column(Integer, primary_key=True)
    only_for_heritage = Column(Boolean)
    fields = relationship("Field", back_populates="entity")
    relationships = relationship("Relationship", back_populates="entity")

class Field(Base):
    __tablename__ = 'field'
    id_ = Column(Integer, primary_key=True)
    name = Column(String)
    type_ = Column(String)
    entity_id = Column(Integer, ForeignKey('entity.id_'))
    is_nullable = Column(Boolean)
    is_primary_key = Column(Boolean)
    is_list = Column(Boolean)
    is_single = Column(Boolean)
    strong = Column(Boolean)
    ordered = Column(Boolean)
    list_model = Column(Boolean)
    list_model_displayed_field = Column(String)
    entity = relationship("Entity", back_populates="fields")

class Feature(Base):
    __tablename__ = 'feature'
    id_ = Column(Integer, primary_key=True)
    name = Column(String)
    use_cases = relationship("UseCase", back_populates="feature")

class Global(Base):
    __tablename__ = 'global'
    id_ = Column(Integer, primary_key=True)
    language = Column(String)
    application_name = Column(String)
    organisation_name = Column(String)
    organisation_domain = Column(String)

class Relationship(Base):
    __tablename__ = 'relationship'
    id_ = Column(Integer, primary_key=True)
    left_entity_name = Column(String)
    right_entity_name = Column(String)
    field_name = Column(String)
    relationship_type = Column(String)
    strength = Column(String)
    direction = Column(String)
    cardinality = Column(String)
    entity_id = Column(Integer, ForeignKey('entity.id_'))
    entity = relationship("Entity", back_populates="relationships")

class Root(Base):
    __tablename__ = 'root'
    id_ = Column(Integer, primary_key=True)
    global_id = Column(Integer, ForeignKey('global.id_'))
    global_ = relationship("Global")
    entities = relationship("Entity")
    features = relationship("Feature")

class UseCase(Base):
    __tablename__ = 'use_case'
    id_ = Column(Integer, primary_key=True)
    name = Column(String)
    validator = Column(Boolean)
    entities = relationship("Entity")
    undoable = Column(Boolean)
    dto_in_id = Column(Integer, ForeignKey('dto.id_'))
    dto_out_id = Column(Integer, ForeignKey('dto.id_'))
    dto_in = relationship("Dto", foreign_keys=[dto_in_id])
    dto_out = relationship("Dto", foreign_keys=[dto_out_id])
    feature_id = Column(Integer, ForeignKey('feature.id_'))
    feature = relationship("Feature", back_populates="use_cases")
