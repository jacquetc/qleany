// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>

namespace Simple::Contracts::DTO::Brand
{

class InsertBrandIntoRelativeDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(int position READ position WRITE setPosition)
    Q_PROPERTY(int relatedId READ relatedId WRITE setRelatedId)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool positionSet = false;
        bool relatedIdSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "id")
            {
                return idSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            if (fieldName == "relatedId")
            {
                return relatedIdSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    InsertBrandIntoRelativeDTO() : m_id(0), m_position(0), m_relatedId(0)
    {
    }

    ~InsertBrandIntoRelativeDTO()
    {
    }

    InsertBrandIntoRelativeDTO(int id, int position, int relatedId)
        : m_id(id), m_position(position), m_relatedId(relatedId)
    {
    }

    InsertBrandIntoRelativeDTO(const InsertBrandIntoRelativeDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_position(other.m_position), m_relatedId(other.m_relatedId)
    {
    }

    bool isValid()
    {
        return m_id > 0;
    }

    InsertBrandIntoRelativeDTO &operator=(const InsertBrandIntoRelativeDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_position = other.m_position;
            m_relatedId = other.m_relatedId;
        }
        return *this;
    }

    friend bool operator==(const InsertBrandIntoRelativeDTO &lhs, const InsertBrandIntoRelativeDTO &rhs);

    friend uint qHash(const InsertBrandIntoRelativeDTO &dto, uint seed) noexcept;

    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
        m_metaData.idSet = true;
    }

    // ------ position : -----

    int position() const
    {
        return m_position;
    }

    void setPosition(int position)
    {
        m_position = position;
        m_metaData.positionSet = true;
    }

    // ------ relatedId : -----

    int relatedId() const
    {
        return m_relatedId;
    }

    void setRelatedId(int relatedId)
    {
        m_relatedId = relatedId;
        m_metaData.relatedIdSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    int m_id;
    int m_position;
    int m_relatedId;
};

inline bool operator==(const InsertBrandIntoRelativeDTO &lhs, const InsertBrandIntoRelativeDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_position == rhs.m_position && lhs.m_relatedId == rhs.m_relatedId;
}

inline uint qHash(const InsertBrandIntoRelativeDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_position, seed);
    hash ^= ::qHash(dto.m_relatedId, seed);

    return hash;
}

} // namespace Simple::Contracts::DTO::Brand
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Brand::InsertBrandIntoRelativeDTO)