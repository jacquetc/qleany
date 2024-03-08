// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_passenger.h"
#include "event_dispatcher.h"
#include "passenger/passenger_interactor.h"

using namespace FrontEnds::Interactor;
using namespace FrontEnds::Presenter;

SinglePassenger::SinglePassenger(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::updated, this, [this](PassengerDTO dto) {
        if (dto.id() == id())
        {

            if (m_id != dto.id())
            {
                m_id = dto.id();
                Q_EMIT idChanged();
            }
            if (m_uuid != dto.uuid())
            {
                m_uuid = dto.uuid();
                Q_EMIT uuidChanged();
            }
            if (m_creationDate != dto.creationDate())
            {
                m_creationDate = dto.creationDate();
                Q_EMIT creationDateChanged();
            }
            if (m_updateDate != dto.updateDate())
            {
                m_updateDate = dto.updateDate();
                Q_EMIT updateDateChanged();
            }
            if (m_name != dto.name())
            {
                m_name = dto.name();
                Q_EMIT nameChanged();
            }
        }
    });
}

int SinglePassenger::id() const
{
    return m_id;
}

void SinglePassenger::setId(int newId)
{
    if (m_id == newId)
        return;
    m_id = newId;
    Q_EMIT idChanged();

    // clear
    if (m_id == 0)
    {

        m_uuid = QUuid{};
        Q_EMIT uuidChanged();

        m_creationDate = QDateTime{};
        Q_EMIT creationDateChanged();

        m_updateDate = QDateTime{};
        Q_EMIT updateDateChanged();

        m_name = QString{};
        Q_EMIT nameChanged();
    }

    // set
    else
    {
        Passenger::PassengerInteractor::instance()->get(m_id).then(
            [this](const FrontEnds::Contracts::DTO::Passenger::PassengerDTO &passenger) {
                if (passenger.isInvalid())
                {
                    qCritical() << Q_FUNC_INFO << "Invalid passengerId";
                    return;
                }

                m_uuid = passenger.uuid();
                Q_EMIT uuidChanged();

                m_creationDate = passenger.creationDate();
                Q_EMIT creationDateChanged();

                m_updateDate = passenger.updateDate();
                Q_EMIT updateDateChanged();

                m_name = passenger.name();
                Q_EMIT nameChanged();
            });
    }
}

void SinglePassenger::resetId()
{
    setId(0);
}

QUuid SinglePassenger::uuid() const
{
    return m_uuid;
}

void SinglePassenger::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;

    UpdatePassengerDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Passenger::PassengerInteractor::instance()->update(dto).then(
        [this](const FrontEnds::Contracts::DTO::Passenger::PassengerDTO &passenger) {
            if (passenger.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid passengerId";
                return;
            }
            m_uuid = passenger.uuid();
            Q_EMIT uuidChanged();
        });
}

QDateTime SinglePassenger::creationDate() const
{
    return m_creationDate;
}

void SinglePassenger::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdatePassengerDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Passenger::PassengerInteractor::instance()->update(dto).then(
        [this](const FrontEnds::Contracts::DTO::Passenger::PassengerDTO &passenger) {
            if (passenger.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid passengerId";
                return;
            }
            m_creationDate = passenger.creationDate();
            Q_EMIT creationDateChanged();
        });
}

QDateTime SinglePassenger::updateDate() const
{
    return m_updateDate;
}

void SinglePassenger::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdatePassengerDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Passenger::PassengerInteractor::instance()->update(dto).then(
        [this](const FrontEnds::Contracts::DTO::Passenger::PassengerDTO &passenger) {
            if (passenger.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid passengerId";
                return;
            }
            m_updateDate = passenger.updateDate();
            Q_EMIT updateDateChanged();
        });
}

QString SinglePassenger::name() const
{
    return m_name;
}

void SinglePassenger::setName(const QString &newName)
{
    if (m_name == newName)
        return;

    UpdatePassengerDTO dto;
    dto.setId(id());
    dto.setName(newName);
    Passenger::PassengerInteractor::instance()->update(dto).then(
        [this](const FrontEnds::Contracts::DTO::Passenger::PassengerDTO &passenger) {
            if (passenger.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid passengerId";
                return;
            }
            m_name = passenger.name();
            Q_EMIT nameChanged();
        });
}
