// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_car_command_handler.h"
#include "car/validators/update_car_command_validator.h"
#include "repository/interface_car_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Car::Commands;
using namespace FrontEnds::Contracts::CQRS::Car::Validators;
using namespace FrontEnds::Application::Features::Car::Commands;

UpdateCarCommandHandler::UpdateCarCommandHandler(InterfaceCarRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarDTO> UpdateCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const UpdateCarCommand &request)
{
    Result<CarDTO> result;

    try {
        result = handleImpl(progressPromise, request);
    } catch (const std::exception &ex) {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateCarCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<CarDTO> UpdateCarCommandHandler::restore()
{
    Result<CarDTO> result;

    try {
        result = restoreImpl();
    } catch (const std::exception &ex) {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateCarCommand restore:" << ex.what();
    }
    return result;
}

Result<CarDTO> UpdateCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise, const UpdateCarCommand &request)
{
    qDebug() << "UpdateCarCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateCarCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(CarDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty()) {
        Result<FrontEnds::Entities::Car> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(CarDTO, currentResult)

        // map
        m_undoState = Result<CarDTO>(FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Car, CarDTO>(currentResult.value()));
    }
    auto updateDto = FrontEnds::Tools::AutoMapper::map<CarDTO, UpdateCarDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto car = FrontEnds::Tools::AutoMapper::map<UpdateCarDTO, FrontEnds::Entities::Car>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty()) {
        car.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto carResult = m_repository->update(std::move(car));

    if (carResult.hasError()) {
        return Result<CarDTO>(carResult.error());
    }

    // map
    auto carDto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Car, CarDTO>(carResult.value());

    Q_EMIT carUpdated(carDto);

    if (request.req.metaData().areDetailsSet()) {
        Q_EMIT carDetailsUpdated(carDto.id());
    }

    qDebug() << "UpdateCarCommandHandler::handleImpl done";

    return Result<CarDTO>(carDto);
}

Result<CarDTO> UpdateCarCommandHandler::restoreImpl()
{
    qDebug() << "UpdateCarCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto car = FrontEnds::Tools::AutoMapper::map<CarDTO, FrontEnds::Entities::Car>(m_undoState.value());

    // do
    auto carResult = m_repository->update(std::move(car));

    QLN_RETURN_IF_ERROR(CarDTO, carResult)

    // map
    auto carDto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Car, CarDTO>(carResult.value());

    Q_EMIT carUpdated(carDto);

    qDebug() << "UpdateCarCommandHandler::restoreImpl done";

    return Result<CarDTO>(carDto);
}

bool UpdateCarCommandHandler::s_mappingRegistered = false;

void UpdateCarCommandHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Car, Contracts::DTO::Car::CarDTO>(true, true);
    FrontEnds::Tools::AutoMapper::registerMapping<Contracts::DTO::Car::UpdateCarDTO, Contracts::DTO::Car::CarDTO>(true, true);
    FrontEnds::Tools::AutoMapper::registerMapping<Contracts::DTO::Car::UpdateCarDTO, FrontEnds::Entities::Car>();
}