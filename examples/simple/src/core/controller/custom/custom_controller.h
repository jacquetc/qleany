// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "custom/write_random_things_dto.h"
#include "event_dispatcher.h"
#include "simple_example_controller_export.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include "custom/get_current_time_reply_dto.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Custom;

namespace Simple::Controller::Custom
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT CustomController : public QObject
{
    Q_OBJECT
  public:
    explicit CustomController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static CustomController *instance();

    Q_INVOKABLE QCoro::Task<GetCurrentTimeReplyDTO> getCurrentTime() const;

  public Q_SLOTS:

    QCoro::Task<> writeRandomThings(WriteRandomThingsDTO dto);

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

} // namespace Simple::Controller::Custom