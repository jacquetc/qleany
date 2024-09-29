#pragma once

#include "error_signals.h"
#include "front_ends_example_controller_export.h"
#include "progress_signals.h"

#include "undo_redo/undo_redo_signals.h"

#include "car/car_signals.h"

#include "brand/brand_signals.h"

#include "passenger/passenger_signals.h"

#include "client/client_signals.h"

#include "custom/custom_signals.h"

#include <QObject>
#include <QPointer>

namespace FrontEnds::Controller
{
class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT EventDispatcher : public QObject
{
    Q_OBJECT
public:
    explicit EventDispatcher();
    static EventDispatcher *instance();

    Q_INVOKABLE ErrorSignals *error() const;
    Q_INVOKABLE ProgressSignals *progress() const;

    Q_INVOKABLE UndoRedoSignals *undoRedo() const;

    Q_INVOKABLE CarSignals *car() const;

    Q_INVOKABLE BrandSignals *brand() const;

    Q_INVOKABLE PassengerSignals *passenger() const;

    Q_INVOKABLE ClientSignals *client() const;

    Q_INVOKABLE CustomSignals *custom() const;

private:
    static QPointer<EventDispatcher> s_instance;
    ErrorSignals *m_errorSignals;
    ProgressSignals *m_progressSignals;

    UndoRedoSignals *m_undoRedoSignals;

    CarSignals *m_carSignals;

    BrandSignals *m_brandSignals;

    PassengerSignals *m_passengerSignals;

    ClientSignals *m_clientSignals;

    CustomSignals *m_customSignals;

    EventDispatcher(const EventDispatcher &) = delete;
    EventDispatcher &operator=(const EventDispatcher &) = delete;
};
} // namespace FrontEnds::Controller