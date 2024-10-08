// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include "entity_base.h"

using namespace DatabaseTest::Entities;

EntityBase::EntityBase() : m_id(0), m_metaData(this)
{
}

EntityBase::~EntityBase()
{
}

EntityBase::EntityBase(int id) : m_id(id), m_metaData(this)
{
}

EntityBase::EntityBase(const EntityBase &other) : m_id(other.m_id), m_metaData(MetaData(this))
{
    m_metaData = MetaData(this, other.metaData());
}

EntityBase &EntityBase::operator=(const EntityBase &other)
{
    if (this != &other)
    {
        m_id = other.m_id;
        m_metaData = MetaData(this, other.metaData());
    }
    return *this;
}

bool EntityBase::isValid() const
{
    return m_id > 0;
}

int EntityBase::id() const
{

    return m_id;
}

void EntityBase::setId(int id)
{
    m_id = id;
}