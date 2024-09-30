// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "custom/write_random_things_dto.h"
#include "event_dispatcher.h"
#include "front_ends_example_controller_export.h"
#include "repository/interface_repository_provider.h"

#include "custom/get_current_time_reply_dto.h"
#include "undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>

using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Controller::UndoRedo;
using namespace FrontEnds::Contracts::DTO::Custom;

namespace FrontEnds::Controller::Custom
{

class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT CustomController : public QObject
{
    Q_OBJECT
public:
    explicit CustomController(InterfaceRepositoryProvider *repositoryProvider,
                              ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static CustomController *instance();

    Q_INVOKABLE QCoro::Task<GetCurrentTimeReplyDTO> getCurrentTime() const;

public Q_SLOTS:

    QCoro::Task<> writeRandomThings(WriteRandomThingsDTO dto);
    WriteRandomThingsDTO getWriteRandomThingsDTO();
    QCoro::Task<> runLongOperation();
    QCoro::Task<> closeSystem();

private:
    static QPointer<CustomController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    CustomController() = delete;
    CustomController(const CustomController &) = delete;
    CustomController &operator=(const CustomController &) = delete;
};

} // namespace FrontEnds::Controller::Custom