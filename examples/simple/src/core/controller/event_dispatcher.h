#pragma once

#include "controller_export.h"
#include "error_signals.h"
#include "passenger/passenger_signals.h"
#include <QObject>

namespace Simple::Controller
{
class SIMPLEEXAMPLE_CONTROLLER_EXPORT EventDispatcher : public QObject
{
    Q_OBJECT
  public:
    explicit EventDispatcher(QObject *parent);
    static EventDispatcher *instance();

    ErrorSignals *error() const;
    PassengerSignals *passenger() const;

  private:
    static EventDispatcher *s_instance;
    ErrorSignals *m_errorSignals;
    PassengerSignals *m_passengerSignals;

    EventDispatcher() = delete;
    EventDispatcher(const EventDispatcher &) = delete;
    EventDispatcher &operator=(const EventDispatcher &) = delete;
};
} // namespace Simple::Controller
