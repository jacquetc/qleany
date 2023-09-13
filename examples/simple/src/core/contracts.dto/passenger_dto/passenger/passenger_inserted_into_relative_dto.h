// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include <QObject>

using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Contracts::DTO::Passenger
{

class PassengerInsertedIntoRelativeDTO
{
    Q_GADGET

    Q_PROPERTY(PassengerDTO passenger READ passenger WRITE setPassenger)
    Q_PROPERTY(int relatedId READ relatedId WRITE setRelatedId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool passengerSet = false;
        bool relatedIdSet = false;
        bool positionSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "passenger")
            {
                return passengerSet;
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

            if (passengerSet)
                return true;

            return false;
        }
    };

    PassengerInsertedIntoRelativeDTO() : m_relatedId(0), m_position(0)
    {
    }

    ~PassengerInsertedIntoRelativeDTO()
    {
    }

    PassengerInsertedIntoRelativeDTO(const PassengerDTO &passenger, int relatedId, int position)
        : m_passenger(passenger), m_relatedId(relatedId), m_position(position)
    {
    }

    PassengerInsertedIntoRelativeDTO(const PassengerInsertedIntoRelativeDTO &other)
        : m_metaData(other.m_metaData), m_passenger(other.m_passenger), m_relatedId(other.m_relatedId),
          m_position(other.m_position)
    {
    }

    PassengerInsertedIntoRelativeDTO &operator=(const PassengerInsertedIntoRelativeDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_passenger = other.m_passenger;
            m_relatedId = other.m_relatedId;
            m_position = other.m_position;
        }
        return *this;
    }

    friend bool operator==(const PassengerInsertedIntoRelativeDTO &lhs, const PassengerInsertedIntoRelativeDTO &rhs);

    friend uint qHash(const PassengerInsertedIntoRelativeDTO &dto, uint seed) noexcept;

    // ------ passenger : -----

    PassengerDTO passenger() const
    {
        return m_passenger;
    }

    void setPassenger(const PassengerDTO &passenger)
    {
        m_passenger = passenger;
        m_metaData.passengerSet = true;
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

    PassengerDTO m_passenger;
    int m_relatedId;
    int m_position;
};

inline bool operator==(const PassengerInsertedIntoRelativeDTO &lhs, const PassengerInsertedIntoRelativeDTO &rhs)
{

    return lhs.m_passenger == rhs.m_passenger && lhs.m_relatedId == rhs.m_relatedId && lhs.m_position == rhs.m_position;
}

inline uint qHash(const PassengerInsertedIntoRelativeDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_passenger, seed);
    hash ^= ::qHash(dto.m_relatedId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Simple::Contracts::DTO::Passenger
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Passenger::PassengerInsertedIntoRelativeDTO)