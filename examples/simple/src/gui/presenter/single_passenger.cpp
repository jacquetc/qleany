// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_passenger.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"

using namespace Simple::Controller;
using namespace Simple::Presenter;

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
                emit idChanged();
            }
            if (m_uuid != dto.uuid())
            {
                m_uuid = dto.uuid();
                emit uuidChanged();
            }
            if (m_creationDate != dto.creationDate())
            {
                m_creationDate = dto.creationDate();
                emit creationDateChanged();
            }
            if (m_updateDate != dto.updateDate())
            {
                m_updateDate = dto.updateDate();
                emit updateDateChanged();
            }
            if (m_name != dto.name())
            {
                m_name = dto.name();
                emit nameChanged();
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
    emit idChanged();

    // clear
    if (m_id == 0)
    {

        m_uuid = QUuid();
        emit uuidChanged();

        m_creationDate = QDateTime();
        emit creationDateChanged();

        m_updateDate = QDateTime();
        emit updateDateChanged();

        m_name = QString();
        emit nameChanged();
    }

    // set
    else
    {
        Passenger::PassengerController::instance()->get(m_id).then(
            [this](const Simple::Contracts::DTO::Passenger::PassengerDTO &passenger) {
                m_uuid = passenger.uuid();
                emit uuidChanged();

                m_creationDate = passenger.creationDate();
                emit creationDateChanged();

                m_updateDate = passenger.updateDate();
                emit updateDateChanged();

                m_name = passenger.name();
                emit nameChanged();
            });
    }
}

void SinglePassenger::resetId()
{
    setId(0);
}

QString SinglePassenger::name() const
{
    return m_name;
}

void SinglePassenger::setName(const QString &newName)
{
    if (m_name == newName)
        return;
    m_name = newName;

    UpdatePassengerDTO dto;
    dto.setId(id());
    dto.setName(newName);
    Passenger::PassengerController::instance()->update(dto);

    emit nameChanged();
}