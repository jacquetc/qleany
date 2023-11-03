// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_car_command_handler.h"
#include "car/validators/update_car_command_validator.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_car_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;
using namespace Simple::Contracts::CQRS::Car::Validators;
using namespace Simple::Application::Features::Car::Commands;

UpdateCarCommandHandler::UpdateCarCommandHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarDTO> UpdateCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const UpdateCarCommand &request)
{
    Result<CarDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateCarCommand:" << ex.what();
    }
    return result;
}

Result<CarDTO> UpdateCarCommandHandler::restore()
{
    Result<CarDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateCarCommand restore:" << ex.what();
    }
    return result;
}

Result<CarDTO> UpdateCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const UpdateCarCommand &request)
{
    qDebug() << "UpdateCarCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateCarCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(CarDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Simple::Domain::Car> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(CarDTO, currentResult)

        // map
        m_undoState = Result<CarDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<CarDTO, UpdateCarDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto car = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateCarDTO, Simple::Domain::Car>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        car.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto carResult = m_repository->update(std::move(car));

    if (carResult.hasError())
    {
        return Result<CarDTO>(carResult.error());
    }

    // map
    auto carDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(carResult.value());

    emit carUpdated(carDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit carDetailsUpdated(carDto.id());
    }

    qDebug() << "UpdateCarCommandHandler::handleImpl done";

    return Result<CarDTO>(carDto);
}

Result<CarDTO> UpdateCarCommandHandler::restoreImpl()
{
    qDebug() << "UpdateCarCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto car = Qleany::Tools::AutoMapper::AutoMapper::map<CarDTO, Simple::Domain::Car>(m_undoState.value());

    // do
    auto carResult = m_repository->update(std::move(car));

    QLN_RETURN_IF_ERROR(CarDTO, carResult)

    // map
    auto carDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(carResult.value());

    emit carUpdated(carDto);

    qDebug() << "UpdateCarCommandHandler::restoreImpl done";

    return Result<CarDTO>(carDto);
}

bool UpdateCarCommandHandler::s_mappingRegistered = false;

void UpdateCarCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true,
                                                                                                             true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Car::UpdateCarDTO,
                                                           Contracts::DTO::Car::CarDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Car::UpdateCarDTO, Simple::Domain::Car>();
}