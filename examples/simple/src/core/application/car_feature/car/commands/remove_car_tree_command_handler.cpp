// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_car_tree_command_handler.h"
#include "repository/interface_car_repository.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;
using namespace Simple::Application::Features::Car::Commands;

RemoveCarTreeCommandHandler::RemoveCarTreeCommandHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveCarTreeCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const RemoveCarTreeCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarTreeCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveCarTreeCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarTreeCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveCarTreeCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const RemoveCarTreeCommand &request)
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

    auto deleteResult = m_repository->removeInCascade(QList<int>() << carId);

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting car from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    // repositories handle remove signals
    // emit carRemoved(deleteResult.value());

    qDebug() << "Car removed:" << carId;

    return Result<int>(carId);
}

Result<int> RemoveCarTreeCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveCarTreeCommandHandler::s_mappingRegistered = false;

void RemoveCarTreeCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true);
}