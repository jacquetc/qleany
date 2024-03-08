// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include <QString>

namespace FrontEnds::Contracts::DTO::Custom
{
using namespace Qt::Literals::StringLiterals;

class WriteRandomThingsDTO
{
    Q_GADGET

    Q_PROPERTY(QString randomCarName READ randomCarName WRITE setRandomCarName)

  public:
    struct MetaData
    {
        bool randomCarNameSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "randomCarName"_L1)
            {
                return randomCarNameSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    WriteRandomThingsDTO() : m_randomCarName(QString())
    {
    }

    ~WriteRandomThingsDTO()
    {
    }

    WriteRandomThingsDTO(const QString &randomCarName) : m_randomCarName(randomCarName)
    {
    }

    WriteRandomThingsDTO(const WriteRandomThingsDTO &other)
        : m_metaData(other.m_metaData), m_randomCarName(other.m_randomCarName)
    {
    }

    WriteRandomThingsDTO &operator=(const WriteRandomThingsDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_randomCarName = other.m_randomCarName;
        }
        return *this;
    }

    WriteRandomThingsDTO &operator=(const WriteRandomThingsDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_randomCarName = other.m_randomCarName;
        }
        return *this;
    }

    WriteRandomThingsDTO &mergeWith(const WriteRandomThingsDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.randomCarNameSet)
            {
                m_randomCarName = other.m_randomCarName;
                m_metaData.randomCarNameSet = true;
            }
        }
        return *this;
    }

    // import operator
    WriteRandomThingsDTO &operator<<(const WriteRandomThingsDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const WriteRandomThingsDTO &lhs, const WriteRandomThingsDTO &rhs);

    friend uint qHash(const WriteRandomThingsDTO &dto, uint seed) noexcept;

    // ------ randomCarName : -----

    QString randomCarName() const
    {
        return m_randomCarName;
    }

    void setRandomCarName(const QString &randomCarName)
    {
        m_randomCarName = randomCarName;
        m_metaData.randomCarNameSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QString m_randomCarName;
};

inline bool operator==(const WriteRandomThingsDTO &lhs, const WriteRandomThingsDTO &rhs)
{

    return lhs.m_randomCarName == rhs.m_randomCarName;
}

inline uint qHash(const WriteRandomThingsDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_randomCarName, seed);

    return hash;
}

} // namespace FrontEnds::Contracts::DTO::Custom
Q_DECLARE_METATYPE(FrontEnds::Contracts::DTO::Custom::WriteRandomThingsDTO)