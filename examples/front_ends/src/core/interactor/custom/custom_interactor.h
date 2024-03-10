// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "custom/write_random_things_dto.h"
#include "event_dispatcher.h"
#include "front_ends_example_interactor_export.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include "custom/get_current_time_reply_dto.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace FrontEnds::Contracts::DTO::Custom;

namespace FrontEnds::Interactor::Custom
{

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT CustomInteractor : public QObject
{
    Q_OBJECT
public:
    explicit CustomInteractor(InterfaceRepositoryProvider *repositoryProvider,
                              ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static CustomInteractor *instance();

    Q_INVOKABLE QCoro::Task<GetCurrentTimeReplyDTO> getCurrentTime() const;

public Q_SLOTS:

    QCoro::Task<> writeRandomThings(WriteRandomThingsDTO dto);
    WriteRandomThingsDTO getWriteRandomThingsDTO();
    QCoro::Task<> runLongOperation();
    QCoro::Task<> closeSystem();

private:
    static QPointer<CustomInteractor> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    CustomInteractor() = delete;
    CustomInteractor(const CustomInteractor &) = delete;
    CustomInteractor &operator=(const CustomInteractor &) = delete;
};

} // namespace FrontEnds::Interactor::Custom