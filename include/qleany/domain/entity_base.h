#pragma once

#include "qleany/qleany_global.h"
#include <QObject>

namespace Qleany::Domain
{

class QLEANY_EXPORT EntityBase
{
    Q_GADGET
    Q_PROPERTY(int id READ id WRITE setId)

  public:
    EntityBase();

    ~EntityBase();
    EntityBase(int id);

    EntityBase(const EntityBase &other);

    EntityBase &operator=(const EntityBase &other);

    bool isValid() const;

    friend bool operator==(const EntityBase &lhs, const EntityBase &rhs);

    friend uint qHash(const EntityBase &entity, uint seed) noexcept;

    // ------ id : -----

    int id() const;

    void setId(int id);

  private:
    int m_id;
};

inline bool operator==(const EntityBase &lhs, const EntityBase &rhs)
{

    return

        lhs.m_id == rhs.m_id;
}

inline uint qHash(const EntityBase &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_id, seed);

    return hash;
}

} // namespace Qleany::Domain
Q_DECLARE_METATYPE(Qleany::Domain::EntityBase)
