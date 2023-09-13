// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Simple::Contracts::DTO::Passenger
{

class CreatePassengerDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(int carId READ carId WRITE setCarId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool nameSet = false;
        bool carIdSet = false;
        bool positionSet = false;
        bool getSet(const QString &fieldName) const
        {
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
            if (fieldName == "name")
            {
                return nameSet;
            }
            if (fieldName == "carId")
            {
                return carIdSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    CreatePassengerDTO()
        : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_name(QString()), m_carId(0),
          m_position(0)
    {
    }

    ~CreatePassengerDTO()
    {
    }

    CreatePassengerDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                       const QString &name, int carId, int position)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_name(name), m_carId(carId),
          m_position(position)
    {
    }

    CreatePassengerDTO(const CreatePassengerDTO &other)
        : m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_name(other.m_name), m_carId(other.m_carId), m_position(other.m_position)
    {
    }

    CreatePassengerDTO &operator=(const CreatePassengerDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_name = other.m_name;
            m_carId = other.m_carId;
            m_position = other.m_position;
        }
        return *this;
    }

    friend bool operator==(const CreatePassengerDTO &lhs, const CreatePassengerDTO &rhs);

    friend uint qHash(const CreatePassengerDTO &dto, uint seed) noexcept;

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

    // ------ name : -----

    QString name() const
    {
        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
        m_metaData.nameSet = true;
    }

    // ------ carId : -----

    int carId() const
    {
        return m_carId;
    }

    void setCarId(int carId)
    {
        m_carId = carId;
        m_metaData.carIdSet = true;
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

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_name;
    int m_carId;
    int m_position;
};

inline bool operator==(const CreatePassengerDTO &lhs, const CreatePassengerDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_name == rhs.m_name && lhs.m_carId == rhs.m_carId &&
           lhs.m_position == rhs.m_position;
}

inline uint qHash(const CreatePassengerDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_name, seed);
    hash ^= ::qHash(dto.m_carId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Simple::Contracts::DTO::Passenger
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Passenger::CreatePassengerDTO)