// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_car_command_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_car_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;
using namespace Simple::Application::Features::Car::Commands;

RemoveCarCommandHandler::RemoveCarCommandHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const RemoveCarCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveCarCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                const RemoveCarCommand &request)
{
    int carId = request.id;

    Result<Simple::Domain::Car> carResult = m_repository->get(carId);

    if (Q_UNLIKELY(carResult.hasError()))
    {
        qDebug() << "Error getting car from repository:" << carResult.error().message();
        return Result<int>(carResult.error());
    }

    // save old entity
    m_oldState = carResult.value();

    auto deleteResult = m_repository->remove(carId);

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting car from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit carRemoved(deleteResult.value());

    qDebug() << "Car removed:" << carId;

    return Result<int>(carId);
}

Result<int> RemoveCarCommandHandler::restoreImpl()
{

    // Add the car to the repository
    auto carResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(carResult.hasError()))
    {
        return Result<int>(carResult.error());
    }

    auto carDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(carResult.value());

    emit carCreated(carDTO);
    qDebug() << "Car added:" << carDTO.id();

    // Return the UUID of the newly created car as a Result object
    return Result<int>(0);
}

bool RemoveCarCommandHandler::s_mappingRegistered = false;

void RemoveCarCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true,
                                                                                                             true);
}