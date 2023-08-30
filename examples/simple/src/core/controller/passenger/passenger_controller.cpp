#include "passenger_controller.h"
#include "passenger/commands/create_passenger_command.h"
#include "passenger/commands/create_passenger_command_handler.h"
#include "passenger/commands/remove_passenger_command.h"
#include "passenger/commands/update_passenger_command.h"
#include "passenger/commands/update_passenger_command_handler.h"
#include "passenger/queries/get_all_passenger_query_handler.h"
#include "passenger/queries/get_passenger_query_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Simple::Controller;
using namespace Simple::Controller::Passenger;
using namespace Simple::Application::Features::Passenger::Commands;
using namespace Simple::Application::Features::Passenger::Queries;

QScopedPointer<PassengerController> PassengerController::s_instance = QScopedPointer<PassengerController>(nullptr);

PassengerController::PassengerController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                         ThreadedUndoRedoSystem *undo_redo_system,
                                         QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{parent}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance.reset(this);
}

PassengerController *PassengerController::instance()
{
    return s_instance.data();
}

void PassengerController::get(int id)
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetPassengerQuery query;
        query.id = id;
        auto interface = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("passenger"));
        GetPassengerQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->passenger()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "passenger");
}

void PassengerController::getWithDetails(int id)
{
}

void PassengerController::getAll()
{
}

QCoro::Task<PassengerDTO> PassengerController::create(const CreatePassengerDTO &dto)
{
    CreatePassengerCommand query;

    query.req = dto;

    auto repository = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("passenger"));

    auto *handler = new CreatePassengerCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreatePassengerCommandHandler::passengerCreated, m_eventDispatcher->passenger(),
                     &PassengerSignals::created);
    QObject::connect(handler, &CreatePassengerCommandHandler::passengerRemoved, this, [this](int removedId) {
        emit m_eventDispatcher->passenger()->removed(QList<int>() << removedId);
    });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreatePassengerCommandHandler, CreatePassengerCommand>(
        PassengerController::tr("Create passenger"), handler, query);

    // push command
    m_undo_redo_system->push(command, "passenger");
    const std::optional<PassengerDTO> optional_result =
        co_await qCoro(handler, &CreatePassengerCommandHandler::passengerCreated, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        co_return PassengerDTO();
    }

    co_return optional_result.value();
}

void PassengerController::update(const UpdatePassengerDTO &dto)
{
}

void PassengerController::remove(int id)
{
}

CreatePassengerDTO PassengerController::getCreateDTO()
{
    return CreatePassengerDTO();
}

UpdatePassengerDTO PassengerController::getUpdateDTO()
{
    return UpdatePassengerDTO();
}
