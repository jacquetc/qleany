// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_passenger_command_handler.h"
#include "passenger/validators/remove_passenger_command_validator.h"
#include "repository/interface_passenger_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;
using namespace Simple::Application::Features::Passenger::Commands;
using namespace Simple::Contracts::CQRS::Passenger::Validators;

RemovePassengerCommandHandler::RemovePassengerCommandHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemovePassengerCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                  const RemovePassengerCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemovePassengerCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<int> RemovePassengerCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemovePassengerCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemovePassengerCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                      const RemovePassengerCommand &request)
{
    int passengerId = request.id;

    // Validate the command using the validator
    auto validator = RemovePassengerCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(passengerId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<Simple::Entities::Passenger> passengerResult = m_repository->get(passengerId);

    QLN_RETURN_IF_ERROR(int, passengerResult)

    // save old entity
    m_oldState = passengerResult.value();

    auto deleteResult = m_repository->remove(QList<int>() << passengerId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove Q_SIGNALS
    // Q_EMIT passengerRemoved(deleteResult.value());

    qDebug() << "Passenger removed:" << passengerId;

    return Result<int>(passengerId);
}

Result<int> RemovePassengerCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemovePassengerCommandHandler::s_mappingRegistered = false;

void RemovePassengerCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Entities::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
}