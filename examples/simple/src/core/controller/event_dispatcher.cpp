#include "event_dispatcher.h"

using namespace Simple::Controller;

EventDispatcher *EventDispatcher::s_instance = nullptr;

EventDispatcher::EventDispatcher(QObject *parent) : QObject{parent}
{
    m_passengerSignals = new PassengerSignals(this);
    m_errorSignals = new ErrorSignals(this);

    s_instance = this;
}

EventDispatcher *EventDispatcher::instance()
{
    return s_instance;
}

PassengerSignals *EventDispatcher::passenger() const
{
    return m_passengerSignals;
}

ErrorSignals *EventDispatcher::error() const
{
    return m_errorSignals;
}
