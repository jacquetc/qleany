#pragma once

#include "controller_export.h"
#include "error_signals.h"

#include "car/car_signals.h"

#include "brand/brand_signals.h"

#include "passenger/passenger_signals.h"

#include "client/client_signals.h"

#include "custom/custom_signals.h"

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

    CarSignals *car() const;

    BrandSignals *brand() const;

    PassengerSignals *passenger() const;

    ClientSignals *client() const;

    CustomSignals *custom() const;

  private:
    static EventDispatcher *s_instance;
    ErrorSignals *m_errorSignals;

    CarSignals *m_carSignals;

    BrandSignals *m_brandSignals;

    PassengerSignals *m_passengerSignals;

    ClientSignals *m_clientSignals;

    CustomSignals *m_customSignals;

    EventDispatcher() = delete;
    EventDispatcher(const EventDispatcher &) = delete;
    EventDispatcher &operator=(const EventDispatcher &) = delete;
};
} // namespace Simple::Controller