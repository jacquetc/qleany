// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "custom_controller.h"

#include "custom/commands/close_system_command.h"
#include "custom/commands/close_system_command_handler.h"
#include "custom/commands/run_long_operation_command.h"
#include "custom/commands/run_long_operation_command_handler.h"
#include "custom/commands/write_random_things_command.h"
#include "custom/commands/write_random_things_command_handler.h"
#include "custom/queries/get_current_time_query.h"
#include "custom/queries/get_current_time_query_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Simple::Controller;
using namespace Simple::Controller::Custom;
using namespace Simple::Application::Features::Custom::Commands;
using namespace Simple::Application::Features::Custom::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<CustomController> CustomController::s_instance = nullptr;

CustomController::CustomController(InterfaceRepositoryProvider *repositoryProvider,
                                   ThreadedUndoRedoSystem *undo_redo_system,
                                   QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance = this;
}

CustomController *CustomController::instance()
{
    return s_instance.data();
}

QCoro::Task<> CustomController::writeRandomThings(WriteRandomThingsDTO dto)
{
    WriteRandomThingsCommand query;

    query.req = dto;

    auto carRepository = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));

    auto passengerRepository =
        static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));

    auto brandRepository = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));

    auto clientRepository = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));

    auto *handler =
        new WriteRandomThingsCommandHandler(carRepository, passengerRepository, brandRepository, clientRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &WriteRandomThingsCommandHandler::writeRandomThingsChanged, m_eventDispatcher->custom(),
                     &CustomSignals::writeRandomThingsChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<WriteRandomThingsCommandHandler, WriteRandomThingsCommand>(
        CustomController::tr("Doing WriteRandomThings"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "custom"_L1);

    co_return;
}

QCoro::Task<> CustomController::runLongOperation()
{
    RunLongOperationCommand query;

    auto *handler = new RunLongOperationCommandHandler();

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &RunLongOperationCommandHandler::runLongOperationChanged, m_eventDispatcher->custom(),
                     &CustomSignals::runLongOperationChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RunLongOperationCommandHandler, RunLongOperationCommand>(
        CustomController::tr("Doing RunLongOperation"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "custom"_L1);

    co_return;
}

QCoro::Task<> CustomController::closeSystem()
{
    CloseSystemCommand query;

    auto carRepository = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));

    auto passengerRepository =
        static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));

    auto brandRepository = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));

    auto clientRepository = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));

    auto *handler =
        new CloseSystemCommandHandler(carRepository, passengerRepository, brandRepository, clientRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &CloseSystemCommandHandler::closeSystemChanged, m_eventDispatcher->custom(),
                     &CustomSignals::closeSystemChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CloseSystemCommandHandler, CloseSystemCommand>(
        CustomController::tr("Doing CloseSystem"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "custom"_L1);

    co_return;
}

QCoro::Task<GetCurrentTimeReplyDTO> CustomController::getCurrentTime() const
{
    auto queryCommand = new QueryCommand("GetCurrentTime"_L1);

    Q_UNIMPLEMENTED();

    queryCommand->setQueryFunction([this](QPromise<Result<void>> &progressPromise) {
        GetCurrentTimeQuery query;

        GetCurrentTimeQueryHandler handler;
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            Q_EMIT m_eventDispatcher->custom()->getCurrentTimeReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "custom"_L1);

    // async wait for result signal
    const std::optional<GetCurrentTimeReplyDTO> optional_result = co_await qCoro(
        m_eventDispatcher->custom(), &CustomSignals::getCurrentTimeReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return GetCurrentTimeReplyDTO();
    }

    co_return optional_result.value();
}
