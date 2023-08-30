// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "domain_export.h"
#include "passenger.h"
#include <QString>

#include "entities.h"
#include "entity.h"

using namespace Qleany::Domain;

namespace Simple::Domain
{

class SIMPLEEXAMPLE_DOMAIN_EXPORT Car : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString content READ content WRITE setContent)

    Q_PROPERTY(Brand brand READ brand WRITE setBrand)

    Q_PROPERTY(bool brandLoaded MEMBER m_brandLoaded)

    Q_PROPERTY(QList<Passenger> passengers READ passengers WRITE setPassengers)

    Q_PROPERTY(bool passengersLoaded MEMBER m_passengersLoaded)

  public:
    Car() : Entity(), m_content(QString())
    {
    }

    ~Car()
    {
    }

    Car(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
        const QString &content, const Brand &brand, const QList<Passenger> &passengers)
        : Entity(id, uuid, creationDate, updateDate), m_content(content), m_brand(brand), m_passengers(passengers)
    {
    }

    Car(const Car &other)
        : Entity(other), m_content(other.m_content), m_brand(other.m_brand), m_brandLoaded(other.m_brandLoaded),
          m_passengers(other.m_passengers), m_passengersLoaded(other.m_passengersLoaded)
    {
    }

    static Simple::Domain::Entities::EntityEnum enumValue()
    {
        return Simple::Domain::Entities::EntityEnum::Car;
    }

    Car &operator=(const Car &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_content = other.m_content;
            m_brand = other.m_brand;
            m_brandLoaded = other.m_brandLoaded;
            m_passengers = other.m_passengers;
            m_passengersLoaded = other.m_passengersLoaded;
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
        if (!m_brandLoaded && m_brandLoader)
        {
            m_brand = m_brandLoader(this->id());
            m_brandLoaded = true;
        }
        return m_brand;
    }

    void setBrand(const Brand &brand)
    {
        m_brand = brand;

        m_brandSet = true;
    }

    using BrandLoader = std::function<Brand(int entityId)>;

    void setBrandLoader(const BrandLoader &loader)
    {
        m_brandLoader = loader;
    }

    bool brandSet() const
    {
        return m_brandSet;
    }

    // ------ passengers : -----

    QList<Passenger> passengers()
    {
        if (!m_passengersLoaded && m_passengersLoader)
        {
            m_passengers = m_passengersLoader(this->id());
            m_passengersLoaded = true;
        }
        return m_passengers;
    }

    void setPassengers(const QList<Passenger> &passengers)
    {
        m_passengers = passengers;

        m_passengersSet = true;
    }

    using PassengersLoader = std::function<QList<Passenger>(int entityId)>;

    void setPassengersLoader(const PassengersLoader &loader)
    {
        m_passengersLoader = loader;
    }

    bool passengersSet() const
    {
        return m_passengersSet;
    }

    static Qleany::Domain::EntitySchema schema;

  private:
    QString m_content;
    Brand m_brand;
    BrandLoader m_brandLoader;
    bool m_brandLoaded = false;
    bool m_brandSet = false;
    QList<Passenger> m_passengers;
    PassengersLoader m_passengersLoader;
    bool m_passengersLoaded = false;
    bool m_passengersSet = false;
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
inline Qleany::Domain::EntitySchema Car::schema = {
    Simple::Domain::Entities::EntityEnum::Car,
    "Car",

    // relationships:
    {{Simple::Domain::Entities::EntityEnum::Car, "Car", Simple::Domain::Entities::EntityEnum::Brand, "Brand", "brand",
      RelationshipType::OneToOne, RelationshipStrength::Strong, RelationshipCardinality::One,
      RelationshipDirection::Forward},
     {Simple::Domain::Entities::EntityEnum::Car, "Car", Simple::Domain::Entities::EntityEnum::Passenger, "Passenger",
      "passengers", RelationshipType::OneToMany, RelationshipStrength::Strong, RelationshipCardinality::ManyOrdered,
      RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"content", FieldType::String, false, false},
     {"brand", FieldType::Entity, false, true},
     {"passengers", FieldType::Entity, false, true}}};

} // namespace Simple::Domain
Q_DECLARE_METATYPE(Simple::Domain::Car)
