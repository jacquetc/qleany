#include "qleany/domain/entity_base.h"

using namespace Qleany::Domain;

EntityBase::EntityBase() : m_id(0)
{
}

EntityBase::~EntityBase()
{
}

EntityBase::EntityBase(int id) : m_id(id)
{
}

EntityBase::EntityBase(const EntityBase &other) : m_id(other.m_id)
{
}

EntityBase &EntityBase::operator=(const EntityBase &other)
{
    if (this != &other)
    {

        m_id = other.m_id;
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
