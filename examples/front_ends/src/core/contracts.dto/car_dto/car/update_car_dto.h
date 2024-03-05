// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "passenger/passenger_dto.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::DTO::Passenger;

namespace FrontEnds::Contracts::DTO::Car
{

class UpdateCarDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)
    Q_PROPERTY(BrandDTO brand READ brand WRITE setBrand)
    Q_PROPERTY(QList<PassengerDTO> passengers READ passengers WRITE setPassengers)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool contentSet = false;
        bool brandSet = false;
        bool passengersSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "id")
            {
                return idSet;
            }
            if (fieldName == "uuid")
            {
                return uuidSet;
            }
            if (fieldName == "creationDate")
            {
                return creationDateSet;
            }
            if (fieldName == "updateDate")
            {
                return updateDateSet;
            }
            if (fieldName == "content")
            {
                return contentSet;
            }
            if (fieldName == "brand")
            {
                return brandSet;
            }
            if (fieldName == "passengers")
            {
                return passengersSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (brandSet)
                return true;

            if (passengersSet)
                return true;

            return false;
        }
    };

    UpdateCarDTO()
        : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_content(QString())
    {
    }

    ~UpdateCarDTO()
    {
    }

    UpdateCarDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                 const QString &content, const BrandDTO &brand, const QList<PassengerDTO> &passengers)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content),
          m_brand(brand), m_passengers(passengers)
    {
    }

    UpdateCarDTO(const UpdateCarDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_content(other.m_content), m_brand(other.m_brand),
          m_passengers(other.m_passengers)
    {
    }

    Q_INVOKABLE bool isValid() const
    {
        return m_id > 0;
    }

    Q_INVOKABLE bool isNull() const
    {
        return !isValid();
    }

    Q_INVOKABLE bool isInvalid() const
    {
        return !isValid();
    }

    UpdateCarDTO &operator=(const UpdateCarDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            m_brand = other.m_brand;
            m_passengers = other.m_passengers;
        }
        return *this;
    }

    UpdateCarDTO &operator=(const UpdateCarDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            m_brand = other.m_brand;
            m_passengers = other.m_passengers;
        }
        return *this;
    }

    UpdateCarDTO &mergeWith(const UpdateCarDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.idSet)
            {
                m_id = other.m_id;
                m_metaData.idSet = true;
            }
            if (other.m_metaData.uuidSet)
            {
                m_uuid = other.m_uuid;
                m_metaData.uuidSet = true;
            }
            if (other.m_metaData.creationDateSet)
            {
                m_creationDate = other.m_creationDate;
                m_metaData.creationDateSet = true;
            }
            if (other.m_metaData.updateDateSet)
            {
                m_updateDate = other.m_updateDate;
                m_metaData.updateDateSet = true;
            }
            if (other.m_metaData.contentSet)
            {
                m_content = other.m_content;
                m_metaData.contentSet = true;
            }
            if (other.m_metaData.brandSet)
            {
                m_brand = other.m_brand;
                m_metaData.brandSet = true;
            }
            if (other.m_metaData.passengersSet)
            {
                m_passengers = other.m_passengers;
                m_metaData.passengersSet = true;
            }
        }
        return *this;
    }

    // import operator
    UpdateCarDTO &operator<<(const UpdateCarDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const UpdateCarDTO &lhs, const UpdateCarDTO &rhs);

    friend uint qHash(const UpdateCarDTO &dto, uint seed) noexcept;

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

    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
        m_metaData.uuidSet = true;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
        m_metaData.creationDateSet = true;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
        m_metaData.updateDateSet = true;
    }

    // ------ content : -----

    QString content() const
    {
        return m_content;
    }

    void setContent(const QString &content)
    {
        m_content = content;
        m_metaData.contentSet = true;
    }

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

    // ------ passengers : -----

    QList<PassengerDTO> passengers() const
    {
        return m_passengers;
    }

    void setPassengers(const QList<PassengerDTO> &passengers)
    {
        m_passengers = passengers;
        m_metaData.passengersSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
    BrandDTO m_brand;
    QList<PassengerDTO> m_passengers;
};

inline bool operator==(const UpdateCarDTO &lhs, const UpdateCarDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_content == rhs.m_content && lhs.m_brand == rhs.m_brand &&
           lhs.m_passengers == rhs.m_passengers;
}

inline uint qHash(const UpdateCarDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_content, seed);
    hash ^= ::qHash(dto.m_brand, seed);
    hash ^= ::qHash(dto.m_passengers, seed);

    return hash;
}

} // namespace FrontEnds::Contracts::DTO::Car
Q_DECLARE_METATYPE(FrontEnds::Contracts::DTO::Car::UpdateCarDTO)