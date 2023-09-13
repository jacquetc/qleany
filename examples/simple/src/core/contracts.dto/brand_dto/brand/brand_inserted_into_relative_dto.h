// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include <QObject>

using namespace Simple::Contracts::DTO::Brand;

namespace Simple::Contracts::DTO::Brand
{

class BrandInsertedIntoRelativeDTO
{
    Q_GADGET

    Q_PROPERTY(BrandDTO brand READ brand WRITE setBrand)
    Q_PROPERTY(int relatedId READ relatedId WRITE setRelatedId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool brandSet = false;
        bool relatedIdSet = false;
        bool positionSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "brand")
            {
                return brandSet;
            }
            if (fieldName == "relatedId")
            {
                return relatedIdSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (brandSet)
                return true;

            return false;
        }
    };

    BrandInsertedIntoRelativeDTO() : m_relatedId(0), m_position(0)
    {
    }

    ~BrandInsertedIntoRelativeDTO()
    {
    }

    BrandInsertedIntoRelativeDTO(const BrandDTO &brand, int relatedId, int position)
        : m_brand(brand), m_relatedId(relatedId), m_position(position)
    {
    }

    BrandInsertedIntoRelativeDTO(const BrandInsertedIntoRelativeDTO &other)
        : m_metaData(other.m_metaData), m_brand(other.m_brand), m_relatedId(other.m_relatedId),
          m_position(other.m_position)
    {
    }

    BrandInsertedIntoRelativeDTO &operator=(const BrandInsertedIntoRelativeDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_brand = other.m_brand;
            m_relatedId = other.m_relatedId;
            m_position = other.m_position;
        }
        return *this;
    }

    friend bool operator==(const BrandInsertedIntoRelativeDTO &lhs, const BrandInsertedIntoRelativeDTO &rhs);

    friend uint qHash(const BrandInsertedIntoRelativeDTO &dto, uint seed) noexcept;

    // ------ brand : -----

    BrandDTO brand() const
    {
        return m_brand;
    }

    void setBrand(const BrandDTO &brand)
    {
        m_brand = brand;
        m_metaData.brandSet = true;
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

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    BrandDTO m_brand;
    int m_relatedId;
    int m_position;
};

inline bool operator==(const BrandInsertedIntoRelativeDTO &lhs, const BrandInsertedIntoRelativeDTO &rhs)
{

    return lhs.m_brand == rhs.m_brand && lhs.m_relatedId == rhs.m_relatedId && lhs.m_position == rhs.m_position;
}

inline uint qHash(const BrandInsertedIntoRelativeDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_brand, seed);
    hash ^= ::qHash(dto.m_relatedId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Simple::Contracts::DTO::Brand
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Brand::BrandInsertedIntoRelativeDTO)