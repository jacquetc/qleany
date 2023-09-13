// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "custom/write_random_things_dto.h"
#include "event_dispatcher.h"
#include "qleany/contracts/repository/interface_repository_provider.h"

#include "custom/get_current_time_reply_dto.h"
#include "qleany/tools/undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QSharedPointer>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Custom;

namespace Simple::Controller::Custom
{

class SIMPLEEXAMPLE_CONTROLLER_EXPORT CustomController : public QObject
{
    Q_OBJECT
  public:
    explicit CustomController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                              ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static CustomController *instance();

  public slots:

    QCoro::Task<> WriteRandomThings(WriteRandomThingsDTO dto);

    QCoro::Task<> CloseSystem();

    QCoro::Task<GetCurrentTimeReplyDTO> GetCurrentTime();

  private:
    static QScopedPointer<CustomController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    CustomController() = delete;
    CustomController(const CustomController &) = delete;
    CustomController &operator=(const CustomController &) = delete;
};

} // namespace Simple::Controller::Custom