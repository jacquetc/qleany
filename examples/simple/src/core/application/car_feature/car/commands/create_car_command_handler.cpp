// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_car_command_handler.h"
#include "car/validators/create_car_command_validator.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Validators;
using namespace Simple::Application::Features::Car::Commands;

CreateCarCommandHandler::CreateCarCommandHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarDTO> CreateCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request)
{
    Result<CarDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateCarCommand:" << ex.what();
    }
    return result;
}

Result<CarDTO> CreateCarCommandHandler::restore()
{
    Result<CarDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateCarCommand restore:" << ex.what();
    }
    return result;
}

Result<CarDTO> CreateCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const CreateCarCommand &request)
{
    qDebug() << "CreateCarCommandHandler::handleImpl called";
    Simple::Domain::Car car;

    if (m_newEntity.isEmpty())
    {
        // Validate the create Car command using the validator
        auto validator = CreateCarCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<CarDTO>(validatorResult.error());
        }

        CreateCarDTO createDTO = request.req;

        // Map the create Car command to a domain Car object and
        // generate a UUID
        car = Qleany::Tools::AutoMapper::AutoMapper::map<CreateCarDTO, Simple::Domain::Car>(createDTO);

        // allow for forcing the uuid
        if (car.uuid().isNull())
        {
            car.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        car.setCreationDate(QDateTime::currentDateTime());
        car.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        car = m_newEntity.value();
    }

    // Add the car to the repository

    m_repository->beginChanges();
    auto carResult = m_repository->add(std::move(car));

    if (Q_UNLIKELY(carResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<CarDTO>(carResult.error());
    }

    m_repository->saveChanges();

    m_newEntity = carResult;

    auto carDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(carResult.value());
    emit carCreated(carDTO);

    qDebug() << "Car added:" << carDTO.id();

    // Return the DTO of the newly created Car as a Result object
    return Result<CarDTO>(carDTO);
}

Result<CarDTO> CreateCarCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newEntity.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Car from repository:" << deleteResult.error().message();
        return Result<CarDTO>(deleteResult.error());
    }

    emit carRemoved(deleteResult.value());

    qDebug() << "Car removed:" << deleteResult.value();

    return Result<CarDTO>(CarDTO());
}

bool CreateCarCommandHandler::s_mappingRegistered = false;

void CreateCarCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Car::CreateCarDTO, Simple::Domain::Car>();
}