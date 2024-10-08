// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "brand_controller.h"

#include "brand/commands/create_brand_command.h"
#include "brand/commands/create_brand_command_handler.h"
#include "brand/commands/remove_brand_command.h"
#include "brand/commands/remove_brand_command_handler.h"
#include "brand/commands/update_brand_command.h"
#include "brand/commands/update_brand_command_handler.h"
#include "brand/queries/get_all_brand_query_handler.h"
#include "brand/queries/get_brand_query_handler.h"
#include "undo_redo/alter_command.h"
#include "undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Simple::Controller;
using namespace Simple::Controller::Brand;
using namespace Simple::Application::Features::Brand::Commands;
using namespace Simple::Application::Features::Brand::Queries;
using namespace Simple::Controller::UndoRedo;
using namespace Simple::Contracts::Repository;

QPointer<BrandController> BrandController::s_instance = nullptr;

BrandController::BrandController(InterfaceRepositoryProvider *repositoryProvider,
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

BrandController *BrandController::instance()
{
    return s_instance.data();
}

QCoro::Task<BrandDTO> BrandController::get(int id) const
{
    auto queryCommand = new QueryCommand("get"_L1);

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetBrandQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));
        GetBrandQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            Q_EMIT m_eventDispatcher->brand()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "brand"_L1);

    // async wait for result signal
    const std::optional<BrandDTO> optional_result =
        co_await qCoro(m_eventDispatcher->brand(), &BrandSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return BrandDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<QList<BrandDTO>> BrandController::getAll() const
{
    auto queryCommand = new QueryCommand("getAll"_L1);

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));
        GetAllBrandQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            Q_EMIT m_eventDispatcher->brand()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "brand"_L1);

    // async wait for result signal
    const std::optional<QList<BrandDTO>> optional_result =
        co_await qCoro(m_eventDispatcher->brand(), &BrandSignals::getAllReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<BrandDTO>() << BrandDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<BrandDTO> BrandController::create(const CreateBrandDTO &dto)
{
    CreateBrandCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));

    auto *handler = new CreateBrandCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateBrandCommandHandler::brandCreated, m_eventDispatcher->brand(),
                     &BrandSignals::created);

    QObject::connect(handler, &CreateBrandCommandHandler::relationWithOwnerInserted, this,
                     [this](int id, int ownerId, int position) {
                         auto dto = CarRelationDTO(ownerId, CarRelationDTO::RelationField::Brand, QList<int>() << id,
                                                   position);
                         Q_EMIT m_eventDispatcher->car()->relationInserted(dto);
                     });
    QObject::connect(handler, &CreateBrandCommandHandler::relationWithOwnerRemoved, this, [this](int id, int ownerId) {
        auto dto = CarRelationDTO(ownerId, CarRelationDTO::RelationField::Brand, QList<int>() << id, -1);
        Q_EMIT m_eventDispatcher->car()->relationRemoved(dto);
    });

    QObject::connect(handler, &CreateBrandCommandHandler::brandRemoved, this,
                     [this](int removedId) { Q_EMIT m_eventDispatcher->brand()->removed(QList<int>() << removedId); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateBrandCommandHandler, CreateBrandCommand>(BrandController::tr("Create brand"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "brand"_L1);

    // async wait for result signal
    const std::optional<BrandDTO> optional_result =
        co_await qCoro(handler, &CreateBrandCommandHandler::brandCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return BrandDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<BrandDTO> BrandController::update(const UpdateBrandDTO &dto)
{
    UpdateBrandCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));

    auto *handler = new UpdateBrandCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateBrandCommandHandler::brandUpdated, this,
                     [this](BrandDTO dto) { Q_EMIT m_eventDispatcher->brand()->updated(dto); });
    QObject::connect(handler, &UpdateBrandCommandHandler::brandDetailsUpdated, m_eventDispatcher->brand(),
                     &BrandSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateBrandCommandHandler, UpdateBrandCommand>(BrandController::tr("Update brand"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "brand"_L1);

    // async wait for result signal
    const std::optional<BrandDTO> optional_result =
        co_await qCoro(handler, &UpdateBrandCommandHandler::brandUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return BrandDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> BrandController::remove(int id)
{
    RemoveBrandCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceBrandRepository *>(m_repositoryProvider->repository("Brand"));

    auto *handler = new RemoveBrandCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveBrandCommandHandler, RemoveBrandCommand>(BrandController::tr("Remove brand"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "brand"_L1);

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateBrandDTO BrandController::getCreateDTO()
{
    return CreateBrandDTO();
}

UpdateBrandDTO BrandController::getUpdateDTO()
{
    return UpdateBrandDTO();
}
