// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_car_command_handler.h"
#include "car/validators/remove_car_command_validator.h"
#include "repository/interface_car_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Car::Commands;
using namespace FrontEnds::Application::Features::Car::Commands;
using namespace FrontEnds::Contracts::CQRS::Car::Validators;

RemoveCarCommandHandler::RemoveCarCommandHandler(InterfaceCarRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const RemoveCarCommand &request)
{
    Result<int> result;

    try {
        result = handleImpl(progressPromise, request);
    } catch (const std::exception &ex) {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<int> RemoveCarCommandHandler::restore()
{
    Result<int> result;

    try {
        result = restoreImpl();
    } catch (const std::exception &ex) {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveCarCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise, const RemoveCarCommand &request)
{
    int carId = request.id;

    // Validate the command using the validator
    auto validator = RemoveCarCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(carId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<FrontEnds::Entities::Car> carResult = m_repository->get(carId);

    QLN_RETURN_IF_ERROR(int, carResult)

    // save old entity
    m_oldState = carResult.value();

    auto deleteResult = m_repository->remove(QList<int>() << carId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove Q_SIGNALS
    // Q_EMIT carRemoved(deleteResult.value());

    qDebug() << "Car removed:" << carId;

    return Result<int>(carId);
}

Result<int> RemoveCarCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveCarCommandHandler::s_mappingRegistered = false;

void RemoveCarCommandHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Car, Contracts::DTO::Car::CarDTO>(true, true);
}