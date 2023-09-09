#include "single_passenger.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"

using namespace Simple::Controller;

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
            setName(dto.name());
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
        setName("");
    }

    // set
    else
    {
        Passenger::PassengerController::instance()->get(m_id).then(
            [this](const Simple::Contracts::DTO::Passenger::PassengerDTO &passenger) { setName(passenger.name()); });
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
