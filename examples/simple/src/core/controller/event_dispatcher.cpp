#include "event_dispatcher.h"

using namespace Simple::Controller;

EventDispatcher *EventDispatcher::s_instance = nullptr;

EventDispatcher::EventDispatcher() : QObject{nullptr}
{
    m_errorSignals = new ErrorSignals(this);
    m_progressSignals = new ProgressSignals(this);
    m_carSignals = new CarSignals(this);
    m_brandSignals = new BrandSignals(this);
    m_passengerSignals = new PassengerSignals(this);
    m_clientSignals = new ClientSignals(this);
    m_customSignals = new CustomSignals(this);

    s_instance = this;
}

EventDispatcher *EventDispatcher::instance()
{
    return s_instance;
}

CarSignals *EventDispatcher::car() const
{
    return m_carSignals;
}

BrandSignals *EventDispatcher::brand() const
{
    return m_brandSignals;
}

PassengerSignals *EventDispatcher::passenger() const
{
    return m_passengerSignals;
}

ClientSignals *EventDispatcher::client() const
{
    return m_clientSignals;
}

CustomSignals *EventDispatcher::custom() const
{
    return m_customSignals;
}

ErrorSignals *EventDispatcher::error() const
{
    return m_errorSignals;
}

ProgressSignals *EventDispatcher::progress() const
{
    return m_progressSignals;
}