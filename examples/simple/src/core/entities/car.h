// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "passenger.h"
#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/entities/entity_schema.h>

using namespace Qleany::Entities;

namespace Simple::Entities
{

class Car : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString content READ content WRITE setContent)

    Q_PROPERTY(Brand brand READ brand WRITE setBrand)

    Q_PROPERTY(QList<Passenger> passengers READ passengers WRITE setPassengers)

  public:
    struct MetaData
    {
        MetaData(Car *entity) : m_entity(entity)
        {
        }
        MetaData(Car *entity, const MetaData &other) : m_entity(entity)
        {
            this->brandSet = other.brandSet;
            this->brandLoaded = other.brandLoaded;
            this->passengersSet = other.passengersSet;
            this->passengersLoaded = other.passengersLoaded;
        }

        bool brandSet = false;
        bool brandLoaded = false;

        bool passengersSet = false;
        bool passengersLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "content")
            {
                return true;
            }
            if (fieldName == "brand")
            {
                return brandSet;
            }
            if (fieldName == "passengers")
            {
                return passengersSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "content")
            {
                return true;
            }
            if (fieldName == "brand")
            {
                return brandLoaded;
            }
            if (fieldName == "passengers")
            {
                return passengersLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Car *m_entity = nullptr;
    };

    Car() : Entity(), m_content(QString()), m_metaData(this)
    {
    }

    ~Car()
    {
    }

    Car(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
        const QString &content, const Brand &brand, const QList<Passenger> &passengers)
        : Entity(id, uuid, creationDate, updateDate), m_content(content), m_brand(brand), m_passengers(passengers),
          m_metaData(this)
    {
    }

    Car(const Car &other)
        : Entity(other), m_metaData(other.m_metaData), m_content(other.m_content), m_brand(other.m_brand),
          m_passengers(other.m_passengers)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Simple::Entities::Entities::EntityEnum enumValue()
    {
        return Simple::Entities::Entities::EntityEnum::Car;
    }

    Car &operator=(const Car &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_content = other.m_content;
            m_brand = other.m_brand;
            m_passengers = other.m_passengers;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Car &lhs, const Car &rhs);

    friend uint qHash(const Car &entity, uint seed) noexcept;

    // ------ content : -----

    QString content() const
    {

        return m_content;
    }

    void setContent(const QString &content)
    {
        m_content = content;
    }

    // ------ brand : -----

    Brand brand()
    {
        if (!m_metaData.brandLoaded && m_brandLoader)
        {
            m_brand = m_brandLoader(this->id());
            m_metaData.brandLoaded = true;
        }
        return m_brand;
    }

    void setBrand(const Brand &brand)
    {
        m_brand = brand;

        m_metaData.brandSet = true;
    }

    using BrandLoader = std::function<Brand(int entityId)>;

    void setBrandLoader(const BrandLoader &loader)
    {
        m_brandLoader = loader;
    }

    // ------ passengers : -----

    QList<Passenger> passengers()
    {
        if (!m_metaData.passengersLoaded && m_passengersLoader)
        {
            m_passengers = m_passengersLoader(this->id());
            m_metaData.passengersLoaded = true;
        }
        return m_passengers;
    }

    void setPassengers(const QList<Passenger> &passengers)
    {
        m_passengers = passengers;

        m_metaData.passengersSet = true;
    }

    using PassengersLoader = std::function<QList<Passenger>(int entityId)>;

    void setPassengersLoader(const PassengersLoader &loader)
    {
        m_passengersLoader = loader;
    }

    static Qleany::Entities::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_content;
    Brand m_brand;
    BrandLoader m_brandLoader;
    QList<Passenger> m_passengers;
    PassengersLoader m_passengersLoader;
};

inline bool operator==(const Car &lhs, const Car &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_content == rhs.m_content && lhs.m_brand == rhs.m_brand && lhs.m_passengers == rhs.m_passengers;
}

inline uint qHash(const Car &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_content, seed);
    hash ^= ::qHash(entity.m_brand, seed);
    hash ^= ::qHash(entity.m_passengers, seed);

    return hash;
}

/// Schema for Car entity
inline Qleany::Entities::EntitySchema Car::schema = {
    Simple::Entities::Entities::EntityEnum::Car,
    "Car",

    // relationships:
    {{Simple::Entities::Entities::EntityEnum::Car, "Car", Simple::Entities::Entities::EntityEnum::Brand, "Brand",
      "brand", RelationshipType::OneToOne, RelationshipStrength::Strong, RelationshipCardinality::One,
      RelationshipDirection::Forward},
     {Simple::Entities::Entities::EntityEnum::Car, "Car", Simple::Entities::Entities::EntityEnum::Passenger,
      "Passenger", "passengers", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"content", FieldType::String, false, false},
     {"brand", FieldType::Entity, false, true},
     {"passengers", FieldType::Entity, false, true}}};

} // namespace Simple::Entities
Q_DECLARE_METATYPE(Simple::Entities::Car)
