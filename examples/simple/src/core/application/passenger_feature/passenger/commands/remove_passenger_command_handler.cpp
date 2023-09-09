// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_passenger_command_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_passenger_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;
using namespace Simple::Application::Features::Passenger::Commands;

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
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemovePassengerCommand:" << ex.what();
    }
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
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemovePassengerCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemovePassengerCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                      const RemovePassengerCommand &request)
{
    int passengerId = request.id;

    Result<Simple::Domain::Passenger> passengerResult = m_repository->get(passengerId);

    if (Q_UNLIKELY(passengerResult.hasError()))
    {
        qDebug() << "Error getting passenger from repository:" << passengerResult.error().message();
        return Result<int>(passengerResult.error());
    }

    // save old entity
    m_oldState = passengerResult.value();

    auto deleteResult = m_repository->remove(passengerId);

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting passenger from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit passengerRemoved(deleteResult.value());

    qDebug() << "Passenger removed:" << passengerId;

    return Result<int>(passengerId);
}

Result<int> RemovePassengerCommandHandler::restoreImpl()
{

    // Add the passenger to the repository
    auto passengerResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(passengerResult.hasError()))
    {
        return Result<int>(passengerResult.error());
    }

    auto passengerDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passengerResult.value());

    emit passengerCreated(passengerDTO);
    qDebug() << "Passenger added:" << passengerDTO.id();

    // Return the UUID of the newly created passenger as a Result object
    return Result<int>(0);
}

bool RemovePassengerCommandHandler::s_mappingRegistered = false;

void RemovePassengerCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
}