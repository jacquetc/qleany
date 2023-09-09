// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include <QDateTime>
#include <QString>
#include <QUuid>




namespace Simple::Contracts::DTO::Car
{

class CarDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)

  public:
    struct MetaData {
    bool idSet = false;
    bool uuidSet = false;
    bool creationDateSet = false;
    bool updateDateSet = false;
    bool contentSet = false;
    bool getSet(const QString &fieldName) const
        {
            if (fieldName == "id")
            {
                return idSet;
            }if (fieldName == "uuid")
            {
                return uuidSet;
            }if (fieldName == "creationDate")
            {
                return creationDateSet;
            }if (fieldName == "updateDate")
            {
                return updateDateSet;
            }if (fieldName == "content")
            {
                return contentSet;
            }
            return false;
        }
    };

    CarDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_content(QString())
    {
    }

    ~CarDTO()
    {
    }

    CarDTO( int id,   const QUuid &uuid,   const QDateTime &creationDate,   const QDateTime &updateDate,   const QString &content ) 
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content)
    {
    }

    CarDTO(const CarDTO &other) : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate), m_content(other.m_content)
    {
    }

    
    bool isValid()
    {
        return m_id > 0;
    }
         

    CarDTO &operator=(const CarDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            
        }
        return *this;
    }

    friend bool operator==(const CarDTO &lhs, const CarDTO &rhs);


    friend uint qHash(const CarDTO &dto, uint seed) noexcept;



    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId( int id)
    {
        m_id = id;
        m_metaData.idSet = true;
    }
    

    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid( const QUuid &uuid)
    {
        m_uuid = uuid;
        m_metaData.uuidSet = true;
    }
    

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate( const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
        m_metaData.creationDateSet = true;
    }
    

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate( const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
        m_metaData.updateDateSet = true;
    }
    

    // ------ content : -----

    QString content() const
    {
        return m_content;
    }

    void setContent( const QString &content)
    {
        m_content = content;
        m_metaData.contentSet = true;
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
};

inline bool operator==(const CarDTO &lhs, const CarDTO &rhs)
{

    return 
            lhs.m_id == rhs.m_id  && lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  && lhs.m_updateDate == rhs.m_updateDate  && lhs.m_content == rhs.m_content 
    ;
}

inline uint qHash(const CarDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_id, seed);
        hash ^= ::qHash(dto.m_uuid, seed);
        hash ^= ::qHash(dto.m_creationDate, seed);
        hash ^= ::qHash(dto.m_updateDate, seed);
        hash ^= ::qHash(dto.m_content, seed);
        

        return hash;
}

} // namespace Simple::Contracts::DTO::Car
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Car::CarDTO)