#include "event_dispatcher.h"

using namespace Simple::Controller;

EventDispatcher *EventDispatcher::s_instance = nullptr;

EventDispatcher::EventDispatcher(QObject *parent) : QObject{parent}
{
    m_passengerSignals = new PassengerSignals(this);
    m_carSignals = new CarSignals(this);
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
CarSignals *EventDispatcher::car() const
{
    return m_carSignals;
}

ErrorSignals *EventDispatcher::error() const
{
    return m_errorSignals;
}
